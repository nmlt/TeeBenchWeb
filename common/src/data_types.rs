use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};
use yewdux::prelude::*;

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
