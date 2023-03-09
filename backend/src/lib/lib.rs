use std::collections::HashMap;
use std::collections::VecDeque;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use time::{Duration, OffsetDateTime};
use tokio::process::Command as TokioCommand;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, instrument, warn};

use common::data_types::{
    Commandline, ExperimentResult, Job, JobStatus, Platform, ProfilingConfiguration, Report,
};

async fn compile_and_run(conf: ProfilingConfiguration) -> Report {
    let tee_bench_dir =
        PathBuf::from(var("TEEBENCHWEB_RUN_DIR").expect("TEEBENCHWEB_RUN_DIR not set"));
    let cmds = conf.to_teebench_cmd();
    let mut tasks = HashMap::new();
    for cmd in cmds {
        let mut tee_bench_dir = tee_bench_dir.clone();
        let cmd_moved = cmd.clone();
        // Each task should get the full "power" of the machine, so don't run them in parallel.
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
            }
            output.stdout
        })
        .await;
        tasks.insert(cmd.to_teebench_args(), handle);
    }
    let mut report = Report {
        config: conf,
        results: vec![],
        findings: vec![],
    };
    for (args, task) in tasks {
        let res = task.unwrap();
        let human_readable = String::from_utf8(res.clone()).unwrap();
        info!("Task output:\n```\n{human_readable}\n```");
        let mut rdr = csv::Reader::from_reader(&*res);
        let mut iter = rdr.deserialize();
        // iter.next(); // First line is skipped anyway because a header is expected.
        let exp_result: ExperimentResult = iter.next().unwrap().unwrap();
        report.results.push((args, exp_result));
    }
    report
}

#[instrument(skip(queue, oneshot_tx, queue_tx))]
async fn work_on_queue(
    queue: Arc<Mutex<VecDeque<Job>>>,
    oneshot_tx: oneshot::Sender<()>,
    queue_tx: mpsc::Sender<Job>,
) {
    fn peek_queue(queue: Arc<Mutex<VecDeque<Job>>>) -> Option<Job> {
        let guard = queue.lock().unwrap();
        guard.front().map(Job::clone)
    }
    // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
    // Probably by making the above function a closure, which would also be more idiomatic.
    while let Some(current_job) = peek_queue(queue.clone()) {
        info!("Working on {current_job:#?}...");
        let report = compile_and_run(current_job.config.clone()).await;
        info!("Process completed with {report:?}.");
        let finished_job = Job {
            status: JobStatus::Done {
                runtime: Duration::new(5, 0), // TODO Get actual runtime from teebench output.
                result: Ok(report),
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
/// - queue_tx: notify webserver of new jobs
async fn receive_confs(
    queue: Arc<Mutex<VecDeque<Job>>>,
    rx: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
    queue_tx: mpsc::Sender<Job>,
) {
    let mut rx_guard = rx.lock().await;
    match rx_guard.recv().await {
        Some(job) => {
            info!("New job came in!");
            queue_tx.send(job.clone()).await.unwrap();
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
/// rx: incoming new profiling configs
/// queue: the actual queue, shared with the server, so it can send the queue to any newly connecting client
/// queue_tx: this channel notifies the server of any changes in the queue.
#[instrument(skip(rx, queue, queue_tx))]
pub async fn profiling_task(
    rx: mpsc::Receiver<Job>,
    queue: Arc<Mutex<VecDeque<Job>>>,
    queue_tx: mpsc::Sender<Job>,
) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    loop {
        receive_confs(queue.clone(), rx.clone(), queue_tx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(
            queue.clone(),
            work_finished_tx,
            queue_tx.clone(),
        ));
        loop {
            // Following the advice in the tokio::oneshot documentation to make the rx &mut.
            tokio::select! {
                _ = receive_confs(queue.clone(), rx.clone(), queue_tx.clone()) => {},
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
