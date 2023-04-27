use anyhow::{bail, Context, Result};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, instrument, warn};

use common::commandline::Commandline;
use common::commit::{CommitState, CompilationStatus};
use common::data_types::{Algorithm, ExperimentChart, Job, JobConfig, JobResult, JobStatus, Report, TeeBenchWebError, REPLACE_ALG, ExperimentType, Measurement, FindingStyle, Parameter, Platform, Dataset};
use common::hardcoded::{hardcoded_perf_report_commands, hardcoded_perf_report_configs};

const BIN_FOLDER: &str = "bin";
const REPLACE_FILE: &str = "Joins/TBW/OperatorJoin.cpp";

// Machine-dependent variables
const CPU_PHYSICAL_CORES: i32 = 8;
// const CPU_LOGICAL_CORES: i32  = 16;
// const L1_SIZE_KB: i32        = 256;
// const L2_SIZE_KB: i32        = 2048;
// const L3_SIZE_KB: i32        = 16384;
// const EPC_SIZE_KB: i32       = 262144; // 256 MB

type SwitchedInType = Arc<tokio::sync::Mutex<Option<Algorithm>>>;

fn display_command_output(o: &std::process::Output, cmd: String) -> String {
    let mut res = String::new();
    res.push_str(&format!("Command `{cmd}` "));
    if o.status.success() {
        res.push_str("succeeded with:\n");
    } else {
        let code = o.status.code();
        res.push_str(&format!("failed (returned {code:?}) with:\n"));
    }
    res.push_str("---STDOUT---\n");
    res.push_str(&String::from_utf8(o.stdout.clone()).unwrap());
    res.push_str("---STDERR---\n");
    res.push_str(&String::from_utf8(o.stderr.clone()).unwrap());
    res.push_str("---END---\n");
    res
}

// TODO Rewrite with a better Command library, eg. duct (if that works for async?). Problems:
// - Commands that return nonzero exit status don't return an Err. The returned result is whether it could even start the command. Makes my code more complicated.
// - Nicer way to get the actual string representing the command for logging.
// - Nicer way to get the interleaved output of stdin and stderr (not the function `display_command_output`!). Might be a problem when compiling SGX: CXX <= File is printed to stdout, compiler errors to stderr.
async fn compile(
    alg: &Algorithm,
    tee_bench_dir: &PathBuf,
    code_hashmap: HashMap<Algorithm, String>,
) -> Result<String> {
    let mut output = String::new();
    let new_code = code_hashmap.get(alg).unwrap();
    let mut replace_file_path = tee_bench_dir.clone();
    replace_file_path.push(REPLACE_FILE);
    tokio::fs::write(replace_file_path, new_code)
        .await
        .with_context(|| format!("Failed to write new operator to {REPLACE_FILE}"))?;
    let compile_args_sgx = ["-B", "sgx"];
    let compile_args_native = ["native", "CFLAGS=-DNATIVE_COMPILATION"];
    let compile_args_native_joined = compile_args_native.join(" ");
    let compile_args_sgx_joined = compile_args_sgx.join(" ");
    let enclave_name = "enclave.signed.so";
    //TokioCommand::new("make").current_dir(tee_bench_dir.clone()).args(["clean"]).status().await.expect("Failed to run make clean");
    let make_out = TokioCommand::new("make")
        .current_dir(tee_bench_dir)
        .args(compile_args_native)
        .output()
        .await
        .with_context(|| format!("Failed to run `make {compile_args_native_joined}`"))?;
    output.push_str(&display_command_output(
        &make_out,
        format!("make {compile_args_native_joined}"),
    ));
    if !make_out.status.success() {
        bail!("Failed to compile native version:\n{output}");
    }
    let (mut app_path, mut bin_path) = (tee_bench_dir.clone(), tee_bench_dir.clone());
    bin_path.push(BIN_FOLDER);
    tokio::fs::create_dir_all(&bin_path)
        .await
        .with_context(|| format!("Failed to create $TEE_BENCH_DIR/{BIN_FOLDER}!"))?;
    app_path.push("app");
    bin_path.push("native");
    info!("Copying from {app_path:?} to {bin_path:?}");
    tokio::fs::copy(&app_path, &bin_path)
        .await
        .context("Failed to copy native binary over!")?;
    bin_path.pop();
    let cmd_out = TokioCommand::new("./native")
        .args(&["-a", REPLACE_ALG]) // TODO Check this and sgx version for output in csv. That's what has to work!
        .current_dir(&bin_path)
        .output()
        .await
        .with_context(|| format!("Failed to run `./native -a {REPLACE_ALG}`"))?;
    output.push_str(&display_command_output(
        &cmd_out,
        format!("./native -a {REPLACE_ALG}"),
    ));
    if !cmd_out.status.success() {
        bail!("Running native example failed with:\n{output}");
    }
    let cmd_out = TokioCommand::new("make")
        .current_dir(tee_bench_dir)
        .args(compile_args_sgx)
        .output()
        .await
        .with_context(|| format!("Failed to run `make {compile_args_sgx_joined}`"))?;
    output.push_str(&display_command_output(
        &cmd_out,
        format!("make {compile_args_sgx_joined}"),
    ));
    if !cmd_out.status.success() {
        bail!("Failed to compile SGX version:\n{output}");
    }
    bin_path.push("sgx");
    info!("Copying from {app_path:?} to {bin_path:?}");
    tokio::fs::copy(&app_path, &bin_path)
        .await
        .context("Failed to copy sgx binary over!")?;
    bin_path.pop();
    bin_path.push(enclave_name);
    app_path.pop();
    app_path.push(enclave_name);
    info!("Copying from {app_path:?} to {bin_path:?}");
    tokio::fs::copy(&app_path, &bin_path)
        .await
        .with_context(|| format!("Failed to copy {enclave_name} over!"))?;
    bin_path.pop();
    let cmd_out = TokioCommand::new("./sgx")
        .args(&["-a", REPLACE_ALG])
        .current_dir(&bin_path)
        .output()
        .await
        .with_context(|| format!("Failed to run ./sgx -a {REPLACE_ALG}"))?;
    output.push_str(&display_command_output(
        &cmd_out,
        format!("./sgx -a {REPLACE_ALG}"),
    ));
    if !cmd_out.status.success() {
        bail!("Running SGX example failed with:\n{output}");
    }
    Ok(output)
}

fn enrich_report_with_findings(jr: &mut Report) {
    // 1. iterate over each experiment chart and enrich it with findings
    for ex in &mut jr.charts {
        match &ex.config {
            JobConfig::Profiling(c) => {
                match c.measurement {
                    Measurement::Throughput=> {
                        match c.parameter {
                            Parameter::Threads => {
                                let throughputs: _ =
                                    ex.get_result_values::<f32>("throughput".to_string(), Platform::Sgx, Dataset::CacheFit);
                                let threads =
                                    ex.get_result_values::<i32>("threads".to_string(), Platform::Sgx, Dataset::CacheFit);
                                let max_result = threads
                                    .iter()
                                    .zip(throughputs.iter())
                                    .enumerate()
                                    .max_by(|(_,a),(_,b)| a.1.partial_cmp(&b.1).unwrap())
                                    .unwrap().1;
                                let max_threads = threads.iter().max().unwrap();

                                jr.findings.push(common::data_types::Finding {
                                    title:"Max Throughput".to_string(),
                                    message: format!("{:?} [M rec/s]", max_result.1),
                                    style: FindingStyle::Good});

                                if max_result.0 + 2 < CPU_PHYSICAL_CORES && max_threads != max_result.0 {
                                    jr.findings.push(common::data_types::Finding {
                                        title:"Very Poor Scalability".to_string(),
                                        message: format!("Used only {:?}/{:?} logical cores", max_result.0, CPU_PHYSICAL_CORES),
                                        style: FindingStyle::Bad});
                                } else if max_result.0 < &CPU_PHYSICAL_CORES && max_threads != max_result.0  {
                                    jr.findings.push(common::data_types::Finding {
                                        title:"Poor Scalability".to_string(),
                                        message: format!("Used only {:?}/{:?} logical cores", max_result.0, CPU_PHYSICAL_CORES),
                                        style: FindingStyle::SoSo});
                                } else {
                                    jr.findings.push(common::data_types::Finding {
                                        title:"Good Scalability".to_string(),
                                        message: format!("Used {:?}/{:?} logical cores", max_result.0, CPU_PHYSICAL_CORES),
                                        style: FindingStyle::Good});
                                }
                                // calculate scalability between pcores and lcores --> add finding if hyper-threading helps
                                // is max throughput close to pcores? --> add finding if the algorithm scales at all
                                // is throughput going down? --> add finding to check CPU context switches
                            }
                            Parameter::DataSkew => {}
                            Parameter::JoinSelectivity => {}
                        }
                    },
                    Measurement::EpcPaging => {}
                }
            },
            JobConfig::PerfReport(c) => {
                match c.exp_type {
                    ExperimentType::EpcPaging => {},
                    ExperimentType::Throughput => {},
                    ExperimentType::Scalability => {},
                    ExperimentType::Custom => {}
                }
            },
            JobConfig::Compile(_) => {}
        }
    }
    // 2. add top-level findings

}

async fn run_experiment(
    tee_bench_dir: PathBuf,
    configs: Vec<JobConfig>,
    cmds: Vec<Vec<Commandline>>,
    code_hashmap: HashMap<Algorithm, String>,
    currently_switched_in: SwitchedInType,
) -> JobResult {
    let mut all_tasks = vec![];
    let mut errors = false;
    for (chart_cmds, conf) in cmds.iter().zip(configs) {
        if errors {
            break;
        }
        let mut cmd_tasks: HashMap<common::data_types::TeebenchArgs, Result<Vec<u8>, _>> =
            HashMap::new();
        for cmd in chart_cmds {
            let mut tee_bench_dir = tee_bench_dir.clone();
            let cmd_moved = cmd.clone();
            let currently_switched_in_moved = currently_switched_in.clone();
            let code_hashmap_moved = code_hashmap.clone();
            // Each task should get the full "power" of the machine, so don't run them in parallel (by awaiting the handle).
            // TODO Clean up clone calls from when we spawned here to not run TB in parallel. That doesn't happen now, right?
            {
                let switched_in = currently_switched_in_moved;
                let mut switched_in = switched_in.lock().await;
                let cmd = cmd_moved.clone();
                let code_hashmap = code_hashmap_moved.clone();
                // This assumes that the Makefile of TeeBench has a different app name ("sgx" or "native"). See `common::data_types::Platform::to_app_name()`.
                if cmd.algorithm.is_commit()
                    && (switched_in.is_none()
                        || (switched_in.is_some() && switched_in.unwrap() != cmd.algorithm))
                {
                    info!("Compiling: {cmd:?}, switched in : {switched_in:?}");
                    if let Err(e) = compile(&cmd.algorithm, &tee_bench_dir, code_hashmap).await {
                        error!("Error while switching in code and compiling commit for experiment:\n{e:#}");
                        return JobResult::Exp(Err(TeeBenchWebError::Compile(format!("{e:#}"))));
                    }
                    switched_in.replace(cmd.algorithm);
                }
                tee_bench_dir.push(BIN_FOLDER);
                let args_key = cmd.to_teebench_args();
                let cmd_string = format!("{cmd}");
                info!("Running `{cmd_string}`");
                let output = to_command(cmd)
                    .current_dir(tee_bench_dir)
                    .output()
                    .await
                    .expect("Failed to run TeeBench");
                if !output.status.success() {
                    error!("Command failed with {output:#?}");
                    cmd_tasks.insert(
                        args_key,
                        Err(TeeBenchWebError::TeeBenchCrash(format!(
                            "Command `{cmd_string}` failed with:\n{output:#?}"
                        ))),
                    );
                    errors = true;
                    break;
                } else {
                    cmd_tasks.insert(args_key, Ok(output.stdout));
                }
            }
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
            let res = match task {
                Ok(res) => res,
                Err(e) => return JobResult::Exp(Err(e)),
            };
            let human_readable = String::from_utf8(res.clone()).unwrap();
            info!("Task output:\n```\n{human_readable}\n```");
            let mut rdr = csv::Reader::from_reader(&*res);
            let mut iter = rdr.deserialize();
            // iter.next(); // First line is skipped anyway because a header is expected.
            let exp_result: HashMap<String, String> = match iter.next() {
                Some(csv_parse_result) => csv_parse_result.expect("Error processing CSV!"),
                None => return JobResult::Exp(Err(TeeBenchWebError::TeeBenchNoOutputData)),
            };
            experiment_chart.results.push((args, exp_result));
        }
        report.charts.push(experiment_chart);
    }

    enrich_report_with_findings(&mut report);

    JobResult::Exp(Ok(report))
}

async fn runner(
    conf: JobConfig,
    commits: Arc<Mutex<CommitState>>,
    currently_switched_in: SwitchedInType,
) -> JobResult {
    let tee_bench_dir =
        PathBuf::from(var("TEEBENCHWEB_RUN_DIR").expect("TEEBENCHWEB_RUN_DIR not set"));
    // TODO Move to JobConfig method.
    let algs = match conf {
        JobConfig::Profiling(ref c) => c.algorithm.clone(),
        JobConfig::PerfReport(ref c) => {
            if let Algorithm::Commit(_) = c.baseline {
                HashSet::from([c.baseline, Algorithm::Commit(c.id)])
            } else {
                HashSet::from([Algorithm::Commit(c.id)])
            }
        }
        JobConfig::Compile(id) => HashSet::from([Algorithm::Commit(id)]),
    };
    let code_hashmap = {
        let guard = commits.lock().unwrap();
        guard.get_used_code(&algs)
    };
    match conf {
        JobConfig::Profiling(ref c) => {
            let cmds = c.to_teebench_cmd();
            let configs = c
                .dataset
                .iter()
                .map(|ds| {
                    let mut clone = c.clone();
                    clone.set_preconfigured_experiment();
                    clone.dataset = HashSet::from([*ds]);
                    JobConfig::Profiling(clone)
                })
                .collect();
            run_experiment(
                tee_bench_dir,
                configs,
                cmds,
                code_hashmap,
                currently_switched_in,
            )
            .await
        }
        JobConfig::PerfReport(ref pr_conf) => {
            let baseline = || {
                let mut guard = commits.lock().unwrap();
                if let Some(mut c) = guard.get_id_mut(&pr_conf.id) {
                    c.perf_report_running = true;
                    return c.baseline;
                }
                panic!("Could not find the commit!");
            };
            let baseline = baseline();
            let cmds = hardcoded_perf_report_commands(pr_conf.id, &baseline);
            let configs = hardcoded_perf_report_configs(pr_conf.id, baseline);
            let results = run_experiment(
                tee_bench_dir,
                configs,
                cmds,
                code_hashmap,
                currently_switched_in,
            )
            .await;
            {
                let mut guard = commits.lock().unwrap();
                if let Some(mut c) = guard.get_id_mut(&pr_conf.id) {
                    c.reports = Some(results.clone());
                    c.perf_report_running = false;
                }
            }
            results
        }
        JobConfig::Compile(ref id) => {
            {
                let mut guard = commits.lock().unwrap();
                if let Some(mut c) = guard.get_id_mut(id) {
                    c.compilation = CompilationStatus::Compiling;
                }
            }
            let result = JobResult::Compile(
                compile(&Algorithm::Commit(*id), &tee_bench_dir, code_hashmap)
                    .await
                    .map_err(|e| e.to_string()),
            );
            {
                let mut guard = commits.lock().unwrap();
                if let Some(mut c) = guard.get_id_mut(id) {
                    c.compilation = match result {
                        JobResult::Compile(Ok(ref msg)) => {
                            CompilationStatus::Successful(msg.clone())
                        }
                        JobResult::Compile(Err(ref msg)) => CompilationStatus::Failed(msg.clone()),
                        _ => unreachable!(),
                    };
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
    commits: Arc<Mutex<CommitState>>,
    currently_switched_in: SwitchedInType,
) {
    fn peek_queue(queue: Arc<Mutex<VecDeque<Job>>>) -> Option<Job> {
        let guard = queue.lock().unwrap();
        guard.front().map(Job::clone)
    }
    // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
    // Probably by making the above function a closure, which would also be more idiomatic.
    while let Some(current_job) = peek_queue(queue.clone()) {
        info!("Working on {current_job:#?}...");
        let result = runner(
            current_job.config.clone(),
            commits.clone(),
            currently_switched_in.clone(),
        )
        .await;
        let result_type = if result.is_ok() {
            "Sucess".to_string()
        } else {
            format!("Failure: {result:?}")
        };
        info!("Process completed: {result_type}.");
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
    commits: Arc<Mutex<CommitState>>,
    queue: Arc<Mutex<VecDeque<Job>>>,
    queue_tx: mpsc::Sender<Job>,
    rx: mpsc::Receiver<Job>,
) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    // TODO Make this just a &mut, Arc should not be needed except if the compiler requires it, but it is never concurrently accessed.
    let currently_switched_in = Arc::new(tokio::sync::Mutex::new(None));
    loop {
        receive_confs(queue.clone(), rx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(
            queue.clone(),
            work_finished_tx,
            queue_tx.clone(),
            commits.clone(),
            currently_switched_in.clone(),
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
