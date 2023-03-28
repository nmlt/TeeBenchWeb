use std::collections::HashMap;
use std::collections::VecDeque;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, instrument, warn};

use common::commandline::Commandline;
use common::data_types::{
    Commit, CompilationStatus, ExperimentChart, Job, JobConfig, JobResult, JobStatus, Platform,
    Report, TeeBenchWebError,
};
use common::hardcoded::{hardcoded_perf_report_commands, hardcoded_perf_report_configs};

async fn run_experiment(
    tee_bench_dir: PathBuf,
    configs: Vec<JobConfig>,
    cmds: Vec<Vec<Commandline>>,
) -> JobResult {
    let mut all_tasks = vec![];
    for (chart_cmds, conf) in cmds.iter().zip(configs) {
        let mut cmd_tasks = HashMap::new();
        for cmd in chart_cmds {
            let mut tee_bench_dir = tee_bench_dir.clone();
            let cmd_moved = cmd.clone();
            // Each task should get the full "power" of the machine, so don't run them in parallel (by awaiting the handle).
            // TODO Do we have to spawn here? I think I just did that when I failed to see the `current_dir` option on `Command`. This was to avoid changing the cwd for the whole axum server.
            let handle = tokio::task::spawn(async move {
                let cmd = cmd_moved.clone();
                match cmd.app {
                    Platform::Sgx => {
                        tee_bench_dir.push("sgx");
                    }
                    Platform::Native => {
                        tee_bench_dir.push("native");
                    }
                }
                // TODO Put the right file into the right folder, if necessary. This requires the JobConfig.
                // This assumes that the Makefile of TeeBench has a different app name ("sgx" or "native"). See `common::data_types::Platform::to_app_name()`.
                // TokioCommand::new("make").current_dir(tee_bench_dir).args(["clean"]).status().await.expect("Failed to run make clean");
                // TokioCommand::new("make").current_dir(tee_bench_dir).args(["-B", "sgx"]).status().await.expect("Failed to compile sgx version of TeeBench");
                let output = to_command(cmd.clone())
                    .current_dir(tee_bench_dir)
                    .output()
                    .await
                    .expect("Failed to run TeeBench");
                info!("Running `{cmd}`");
                if !output.status.success() {
                    error!("Command failed with {output:#?}");
                    return Err(());
                }
                Ok(output.stdout)
            })
            .await;
            cmd_tasks.insert(cmd.to_teebench_args(), handle);
        }
        all_tasks.push((conf, cmd_tasks));
    }
    let mut report = Report {
        charts: vec![],
        findings: vec![],
    };
    for (conf, tasks) in all_tasks {
        let mut experiment_chart = ExperimentChart::new(conf, vec![], vec![]);
        for (args, task) in tasks {
            let Ok(res) = task.unwrap() else {
                return JobResult::Exp(Err(TeeBenchWebError::default()));
            };
            let human_readable = String::from_utf8(res.clone()).unwrap();
            info!("Task output:\n```\n{human_readable}\n```");
            let mut rdr = csv::Reader::from_reader(&*res);
            let mut iter = rdr.deserialize();
            // iter.next(); // First line is skipped anyway because a header is expected.
            let exp_result: HashMap<String, String> = iter.next().unwrap().unwrap();
            experiment_chart.results.push((args, exp_result));
        }
        report.charts.push(experiment_chart);
    }
    JobResult::Exp(Ok(report))
}

async fn compile_and_run(conf: JobConfig, commits: Arc<Mutex<Vec<Commit>>>) -> JobResult {
    let tee_bench_dir =
        PathBuf::from(var("TEEBENCHWEB_RUN_DIR").expect("TEEBENCHWEB_RUN_DIR not set"));
    match conf {
        JobConfig::Profiling(ref c) => {
            let cmds = c.to_teebench_cmd();
            run_experiment(tee_bench_dir, vec![conf], vec![cmds]).await
        }
        JobConfig::PerfReport(ref pr_conf) => {
            let baseline = || {
                let mut guard = commits.lock().unwrap();
                // TODO I'm repeating this code too often (eg. 20 lines down). Make finding commit with id a function.
                for c in guard.iter_mut() {
                    if c.id == pr_conf.id {
                        c.perf_report_running = true;
                        return c.baseline;
                    }
                }
                panic!("Could not find the commit!");
            };
            let baseline = baseline();
            let cmds = hardcoded_perf_report_commands(&baseline);
            let confs = hardcoded_perf_report_configs(pr_conf.id, baseline);
            let results = run_experiment(tee_bench_dir, confs, cmds).await;
            {
                let mut guard = commits.lock().unwrap();
                for c in guard.iter_mut() {
                    if c.id == pr_conf.id {
                        c.reports = Some(results.clone());
                        c.perf_report_running = false;
                        break;
                    }
                }
            }
            results
        }
        JobConfig::Compile(id) => {
            {
                let mut guard = commits.lock().unwrap();
                // TODO I'm repeating this code too often (eg. 20 lines down). Make finding commit with id a function.
                for c in guard.iter_mut() {
                    if c.id == id {
                        c.compilation = CompilationStatus::Compiling;
                        break;
                    }
                }
            }
            // TODO find file with `id`
            // put it in the right place in teebench src dir
            // OR: give this function access to CommitState (`ServerState`). That is probably the way to go :(
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let output;
            let result = if true {
                output = "warning: This is a placeholder warning".to_string();
                JobResult::Compile(Ok(output))
            } else {
                output = "Fortuna wasn't merciful this time".to_string();
                JobResult::Compile(Err(output))
            };
            {
                let mut guard = commits.lock().unwrap();
                for c in guard.iter_mut() {
                    if c.id == id {
                        c.compilation = match result {
                            JobResult::Compile(Ok(ref msg)) => {
                                CompilationStatus::Successful(msg.clone())
                            }
                            JobResult::Compile(Err(ref msg)) => {
                                CompilationStatus::Failed(msg.clone())
                            }
                            _ => unreachable!(),
                        };
                    }
                }
            }
            result
        }
    }
}

#[instrument(skip(queue, oneshot_tx, queue_tx, commits))]
async fn work_on_queue(
    queue: Arc<Mutex<VecDeque<Job>>>,
    oneshot_tx: oneshot::Sender<()>,
    queue_tx: mpsc::Sender<Job>,
    commits: Arc<Mutex<Vec<Commit>>>,
) {
    fn peek_queue(queue: Arc<Mutex<VecDeque<Job>>>) -> Option<Job> {
        let guard = queue.lock().unwrap();
        guard.front().map(Job::clone)
    }
    // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
    // Probably by making the above function a closure, which would also be more idiomatic.
    while let Some(current_job) = peek_queue(queue.clone()) {
        info!("Working on {current_job:#?}...");
        let result = compile_and_run(current_job.config.clone(), commits.clone()).await;
        info!("Process completed with {result:?}.");
        let finished_job = Job {
            status: JobStatus::Done {
                runtime: Duration::new(5, 0), // TODO Get actual runtime from teebench output.
                result,
            },
            ..current_job
        };
        {
            let mut guard = queue.lock().unwrap();
            guard.pop_front();
        }
        queue_tx.send(finished_job).await.unwrap();
    }
    oneshot_tx.send(()).unwrap();
}

/// Receives the new job and notifies websocket about the new job
// TODO that's weird... The websocket received the job, so why can't it react to the new job without a notification from here? So that it can select! what to do?
///
/// - queue: still the actual queue, to queue new jobs
/// - rx: channel to webserver, to receive new jobs from the websocket
async fn receive_confs(
    queue: Arc<Mutex<VecDeque<Job>>>,
    rx: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
) {
    let mut rx_guard = rx.lock().await;
    match rx_guard.recv().await {
        Some(job) => {
            info!("New job came in!");
            let mut guard = queue.lock().unwrap();
            guard.push_back(job);
        }
        None => warn!("Received None!"), // TODO Why would this happen?
    }
}

/// Runs and compiles the all the experiments, and sends the results back to the server (which sends the results to the client).
///
/// Wait for new jobs in a loop. When a new job arrives, queue it and start a task to work on the queue. Then start a new loop to receive new jobs or receive the notification that there are no more jobs in the queue. When that notification arrives, break that loop and restart the outer one.
///
/// commits: The list of commits uploaded, to compile and generate perf reports for them.
/// queue: the actual queue, shared with the server, so it can send the queue to any newly connecting client
/// queue_tx: this channel notifies the server of any changes in the queue.
/// rx: incoming new profiling configs
#[instrument(skip(commits, queue, queue_tx, rx))]
pub async fn profiling_task(
    commits: Arc<Mutex<Vec<Commit>>>,
    queue: Arc<Mutex<VecDeque<Job>>>,
    queue_tx: mpsc::Sender<Job>,
    rx: mpsc::Receiver<Job>,
) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    loop {
        receive_confs(queue.clone(), rx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(
            queue.clone(),
            work_finished_tx,
            queue_tx.clone(),
            commits.clone(),
        ));
        loop {
            // Following the advice in the tokio::oneshot documentation to make the rx &mut.
            tokio::select! {
                _ = receive_confs(queue.clone(), rx.clone()) => {},
                _ = &mut work_finished_rx => { break; },
            }
        }
    }
}

/// Consumes `Commandline` and makes a tokio process out of it.
fn to_command(cmdline: Commandline) -> TokioCommand {
    let mut cmd = TokioCommand::new(cmdline.app.to_app_name());
    cmd.args(cmdline.args);
    cmd
}
