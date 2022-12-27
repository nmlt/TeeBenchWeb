use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use indoc::writedoc;
pub use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};
use thiserror::Error;
use yewdux::prelude::Store;

use std::collections::HashSet;

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TeeBenchWebError {
    #[error("Could not retrieve results")]
    NoResults,
    #[error("Unknown error")]
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub title: String,
    pub datetime: OffsetDateTime,
    pub code: String,
    pub report: Option<Report>,
    // TODO Add an ID that the server generates to uniquely identify a commit, indenpendently of the user supplied title.
}

impl Commit {
    pub fn new(
        title: String,
        datetime: OffsetDateTime,
        code: String,
        report: Option<Report>,
    ) -> Self {
        Commit {
            title,
            datetime,
            code,
            report,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Report {
    pub performance_gain: u32,
    pub epc_pages: u32,
}

pub type JobResult = Result<Report, TeeBenchWebError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Job {
    Running(ProfilingConfiguration),
    Finished {
        config: ProfilingConfiguration, // TODO This doesn't even have to be here. We know it's the first item in the queue. Better to be sure, I guess.
        submitted: OffsetDateTime,
        runtime: Duration,
        result: JobResult,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueMessage {
    /// Frontend wants to get the current queue
    RequestQueue, // TODO Do I even need that? Can the server just send its queue when the socket opens
    /// Frontend wants to clear the queue
    RequestClear,
    // Frontend received message
    Acknowledge, // TODO Can I trust that transmission succeeds?
    // TODO Either merge the next two messages and use the Job enum or remove that enum.
    /// Backend has a new job (or the frontend just requested the queue)
    /// This message gets send for each item in the queue.
    AddQueueItem(ProfilingConfiguration),
    /// Backend has finished the current top queue item and wants the frontend to remove it from the queue.
    /// Also the frontend should add the attached JobResult to that Job.
    RemoveQueueItem(Job),
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    EnumString,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Algorithm {
    #[default]
    Rho,
    Cht,
    #[strum(to_string = "Latest Commit")]
    Commit(u32), // TODO Uuid
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum ExperimentType {
    #[default]
    #[strum(to_string = "EPC Paging")]
    EpcPaging,
    Throughput,
    #[strum(to_string = "CPU Cycles/Tuple")]
    CpuCyclesTuple,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Parameter {
    #[default]
    Threads,
    DataSkew,
    JoinSelectivity,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Dataset {
    #[default]
    CacheFit,
    CacheExceed,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Platform {
    #[default]
    #[strum(to_string = "SGX")]
    Sgx,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Store)]
pub struct ProfilingConfiguration {
    pub algorithm: HashSet<Algorithm>,
    pub experiment_type: ExperimentType,
    pub parameter: Parameter,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    pub dataset: Dataset,
    pub platform: Platform,
    pub sort_data: bool,
}

impl std::fmt::Display for ProfilingConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            "
            Configuration:
                Algorithm(s): {:?}
                Experiment Type: {}
                Parameter: {}
                min: {}
                max: {}
                step: {}
                Dataset: {}
                Platform: {}
                Pre-sort data: {}
        ",
            self.algorithm,
            self.experiment_type,
            self.parameter,
            self.min,
            self.max,
            self.step,
            self.dataset,
            self.platform,
            self.sort_data
        )
    }
}
