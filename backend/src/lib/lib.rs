use std::collections::VecDeque;
use std::env::{set_current_dir, var};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use time::{Duration, OffsetDateTime};
use tokio::process::Command;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, instrument, warn};

use common::data_types::{
    Commandline, ExperimentType, Job, Platform, ProfilingConfiguration, Report, ReportWithFindings,
};

async fn compile_and_run(conf: ProfilingConfiguration) -> Report {
    let mut tee_bench_dir =
        PathBuf::from(var("TEEBENCHWEB_RUN_DIR").expect("TEEBENCHWEB_RUN_DIR not set"));
    let cmds = conf.to_teebench_cmd();
    let mut outputs = vec![];
    for cmd in cmds {
        // match cmd.app {
        //     Platform::Sgx => {
        //         tee_bench_dir.push("sgx");
        //     }
        //     Platform::Native => {
        //         tee_bench_dir.push("native");
        //     }
        // }
        set_current_dir(&tee_bench_dir).expect("Failed to change to TeeBench dir");
        // Command::new("make").args(["clean"]).status().await.expect("Failed to run make clean");
        // Command::new("make").args(["-B", "sgx"]).status().await.expect("Failed to compile sgx version of TeeBench");
        // This assumes that the Makefile of TeeBench has a different app name ("sgx")
        info!("Now running `{cmd}`...");
        let output = to_command(cmd)
            .output()
            .await
            .expect("Failed to run TeeBench");
        outputs.push(output);
    }
    println!("{outputs:#?}");
    Report::default()
}

#[instrument(skip(rx, queue, queue_tx))]
pub async fn profiling_task(
    rx: mpsc::Receiver<ProfilingConfiguration>,
    queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
    queue_tx: mpsc::Sender<Job>,
) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    #[instrument(skip(queue, oneshot_tx, queue_tx))]
    async fn work_on_queue(
        queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        oneshot_tx: oneshot::Sender<()>,
        queue_tx: mpsc::Sender<Job>,
    ) {
        fn pop_queue(
            queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        ) -> Option<ProfilingConfiguration> {
            let mut guard = queue.lock().unwrap();
            guard.pop_front()
        }
        // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
        // Probably by making the above function a closure, which would also be more idiomatic.
        while let Some(current_conf) = pop_queue(queue.clone()) {
            info!("Working on {current_conf:#?}...");
            let report = compile_and_run(current_conf.clone()).await;
            info!("Process completed with {report:?}.");
            let finished_job = Job::Finished {
                config: current_conf,
                submitted: OffsetDateTime::now_utc(), // TODO Fix this.
                runtime: Duration::new(5, 0), // TODO Get actual runtime from teebench output.
                result: Ok(ReportWithFindings {
                    report: report,
                    findings: Vec::new(),
                }),
            };
            queue_tx.send(finished_job).await.unwrap();
        }
        oneshot_tx.send(()).unwrap();
    }
    async fn receive_confs(
        queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        rx: Arc<tokio::sync::Mutex<mpsc::Receiver<ProfilingConfiguration>>>,
        queue_tx: mpsc::Sender<Job>,
    ) {
        let mut rx_guard = rx.lock().await;
        match rx_guard.recv().await {
            Some(conf) => {
                info!("New task came in!");
                let running_job = Job::Running(conf.clone());
                queue_tx.send(running_job).await.unwrap();
                let mut guard = queue.lock().unwrap();
                guard.push_back(conf);
            }
            None => warn!("Received None!"), // TODO Why would this happen?
        }
    }
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
fn to_command(cmdline: Commandline) -> tokio::process::Command {
    let mut cmd = Command::new(cmdline.app_string());
    cmd.args(cmdline.args);
    cmd
}
