use tokio::sync::{mpsc, oneshot};
use tracing::{info, instrument, warn};
use time::{Duration, OffsetDateTime};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

use common::data_types::{ExperimentType, Job, ProfilingConfiguration, ReportWithFindings, Report, Commandline};

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
            let out = tokio::process::Command::new("sleep")
                .arg("2")
                .output()
                .await;
            info!("Process completed with {out:?}.");
            let report = match current_conf.experiment_type {
                ExperimentType::EpcPaging => Report::Epc { findings: vec![] },
                ExperimentType::Throughput => Report::Throughput { findings: vec![] },
                ExperimentType::CpuCyclesTuple => Report::Scalability { findings: vec![] }, // TODO This is probably wrong?
            };
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
    let mut cmd = tokio::process::Command::new(cmdline.app);
    cmd.args(cmdline.args);
    cmd
}
