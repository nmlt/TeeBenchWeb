use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};
use yewdux::prelude::Store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub title: String,
    pub datetime: DateTime<Utc>,
    pub code: String,
    pub report: Option<Report>,
    // TODO Add an ID that the server generates to uniquely identify a commit, indenpendently of the user supplied title.
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Report {
    pub performance_gain: u32,
}

type JobResult = Option<Report>; // TODO Maybe make this a Result in case the job failed.

/// This type is probably misnamed. It is a JobResult and the ProfilingConfiguration represents a job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Job {
    config: ProfilingConfiguration,
    result: JobResult, 
}

// impl Job {
//     fn with_config(config: ProfilingConfiguration) -> Self {
//         Self {
//             config,
//             result: None,
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueMessage {
    /// Frontend wants to get the current queue
    RequestQueue, // TODO Do I even need that? Can the server just send its queue when the socket opens
    /// Frontend wants to clear the queue
    RequestClear,
    // Frontend received message
    Acknowledge, // TODO Can I trust that transmission succeeds?
    /// Backend has a new job (or the frontend just requested the queue)
    /// This message gets send for each item in the queue.
    AddQueueItem(Job),
    /// Backend has finished the current top queue item and wants the frontend to remove it from the queue.
    /// Also the frontend should add the attached JobResult to that Job.
    RemoveQueueItem(JobResult),
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Algorithm {
    #[default]
    Rho,
    Cht,
    #[strum(to_string = "Commit")]
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
    pub algorithm: Algorithm,
    pub experiment_type: ExperimentType,
    pub parameter: Parameter,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    pub dataset: Dataset,
    pub platform: Platform,
    pub sort_data: bool,
}
