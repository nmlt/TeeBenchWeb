use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use indoc::writedoc;
pub use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumIter, EnumVariantNames};
use thiserror::Error;
use yewdux::prelude::Store;

use std::collections::HashSet;
use std::fmt::Display;

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TeeBenchWebError {
    #[error("Could not retrieve results")]
    NoResults,
    #[error("Unknown error")]
    #[default]
    Unknown,
}

/// A commit represents an algorithm and its profiling results.
///
/// The `reports` field contain all profiling jobs that included this algorithm. So if a profiling job compared algorithm A and B, the both commits' `report` field contains the result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub title: String,
    pub operator: String,
    pub datetime: OffsetDateTime,
    pub code: String,
    pub reports: Vec<Report>,
    // TODO Add an ID that the server generates to uniquely identify a commit, independently of the user supplied title.
}

impl Commit {
    pub fn new(
        title: String,
        operator: String,
        datetime: OffsetDateTime,
        code: String,
        reports: Vec<Report>,
    ) -> Self {
        Commit {
            title,
            operator,
            datetime,
            code,
            reports,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Report {
    Epc { findings: Vec<Finding> },
    Scalability { findings: Vec<Finding> },
    ScalabilityNativeSgxExample { findings: Vec<Finding> },
    Throughput { findings: Vec<Finding> },
    EpcCht { findings: Vec<Finding> },
}

impl Default for Report {
    fn default() -> Self {
        Report::Epc { findings: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Hash, Eq)]
pub enum FindingStyle {
    #[default]
    Neutral,
    Good,
    SoSo,
    Bad,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct Finding {
    pub title: String,
    pub message: String,
    pub style: FindingStyle,
}

impl Finding {
    pub fn new(title: &str, message: &str, style: FindingStyle) -> Self {
        Self {
            title: title.to_owned(),
            message: message.to_owned(),
            style,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Store)]
pub struct ReportWithFindings {
    pub report: Report,
    pub findings: Vec<Finding>,
}

pub type JobResult = Result<ReportWithFindings, TeeBenchWebError>;

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
    EnumIter,
    Eq,
    Hash,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Algorithm {
    #[default]
    // #[strum(to_string = "JOIN - CHT")]
    Cht,
    // #[strum(to_string = "JOIN - PHT")]
    Pht,
    // #[strum(to_string = "JOIN - PSM")]
    Psm,
    // #[strum(to_string = "JOIN - MWAY")]
    Mway,
    // #[strum(to_string = "JOIN - RHT")]
    Rht,
    // #[strum(to_string = "JOIN - RHO")]
    Rho,
    // #[strum(to_string = "JOIN - RSM")]
    Rsm,
    // #[strum(to_string = "JOIN - INL")]
    Inl,
    // #[strum(to_string = "JOIN - v2.1")]
    V21,
    // #[strum(to_string = "JOIN - v2.2")]
    V22,
    // #[strum(to_string = "JOIN - NestedLoopJoin")]
    Nlj,
    //     #[strum(to_string = "Latest Commit")]
    //     Commit(u32), // TODO Uuid
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
    Custom,
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
pub enum Measurement {
    #[default]
    Throughput,
    EpcPaging,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Dataset {
    #[default]
    CacheFit,
    CacheExceed,
}

impl Dataset {
    pub fn to_cmd_arg(&self) -> String {
        match self {
            Dataset::CacheFit => "cache-fit".to_string(),
            Dataset::CacheExceed => "cache-exceed".to_string(),
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    EnumVariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Platform {
    #[default]
    #[strum(to_string = "SGX")]
    Sgx,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Store)]
pub struct ProfilingConfiguration {
    pub algorithm: HashSet<Algorithm>,
    pub experiment_type: ExperimentType,
    pub parameter: Parameter,
    pub measurement: Measurement,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    pub dataset: HashSet<Dataset>,
    pub platform: HashSet<Platform>,
    pub sort_data: bool,
}

impl ProfilingConfiguration {
    pub fn new(
        algorithm: Vec<Algorithm>,
        experiment_type: ExperimentType,
        parameter: Parameter,
        measurement: Measurement,
        min: i64,
        max: i64,
        step: i64,
        d: Vec<Dataset>,
        p: Vec<Platform>,
        sort_data: bool,
    ) -> Self {
        let mut alg = HashSet::new();
        for a in algorithm {
            alg.insert(a);
        }
        let mut dataset = HashSet::new();
        for ds in d {
            dataset.insert(ds);
        }
        let mut platform = HashSet::new();
        for pl in p {
            platform.insert(pl);
        }
        Self {
            algorithm: alg,
            experiment_type,
            parameter,
            measurement,
            min,
            max,
            step,
            dataset,
            platform,
            sort_data,
        }
    }
}
use strum::IntoEnumIterator;
impl Default for ProfilingConfiguration {
    /// Like Figure 4 off the shelf performance: throughput of all algorithms.
    fn default() -> Self {
        let algorithm = Algorithm::iter().collect();
        Self {
            algorithm,
            experiment_type: ExperimentType::default(),
            parameter: Parameter::default(),
            measurement: Measurement::default(),
            min: 2,
            max: 2,
            step: 1,
            dataset: HashSet::from([Dataset::CacheExceed, Dataset::CacheFit]),
            platform: HashSet::from([Platform::default()]),
            sort_data: false,
        }
    }
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
                Dataset: {:?}
                Platform: {:?}
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

impl ProfilingConfiguration {
    pub fn to_teebench_cmd(&self) -> Vec<Commandline> {
        let mut res = vec![];
        for platform in &self.platform {
            let cmd = Commandline::new(*platform);
            res.push(cmd);
        }
        // ProfilingConfiguration.experiment_type only relevant for output
        // ProfilingConfiguration.measurement only relevant for output
        let mut alg_iter = self.algorithm.iter();
        let alg = alg_iter.next().unwrap(); // There is at least one algorithm.
        for cmd in &mut res {
            cmd.add_args("-a", alg);
        }
        Commandline::double_cmds_with_different_arg_value(&mut res, &mut alg_iter);

        if self.sort_data {
            for cmd in &mut res {
                cmd.add_args("--sort-r", "--sort-s");
            }
        }
        let mut dataset_iter = self.dataset.iter();
        let ds = dataset_iter.next().unwrap(); // There is always at least one dataset in a ProfilingConfiguration.
        for cmd in &mut res {
            cmd.add_args("-d", ds.to_cmd_arg());
        }
        Commandline::double_cmds_with_different_arg_value(&mut res, &mut dataset_iter.map(|ds| ds.to_cmd_arg()));
        let mut value_iter = (self.min..=self.max).step_by(self.step as usize); // TODO Verify that these values form a valid range.
        let val = value_iter.next().unwrap();
        let p = match self.parameter {
            Parameter::Threads => "-n",
            Parameter::DataSkew => "-z",
            Parameter::JoinSelectivity => "-l",
        };
        for cmd in &mut res {
            cmd.add_args(p, val);
        }
        Commandline::double_cmds_with_different_arg_value(&mut res, &mut value_iter);

        res
    }
    pub fn set_preconfigured_experiment(&mut self) {
        match self.experiment_type {
            ExperimentType::Throughput => {
                self.parameter = Parameter::Threads;
                self.measurement = Measurement::Throughput;
                self.min = 2;
                self.max = 2;
                self.step = 1;
                self.dataset = HashSet::from([Dataset::CacheExceed, Dataset::CacheFit]);
                self.platform = HashSet::from([Platform::Sgx]);
                self.sort_data = false;
            },
            ExperimentType::EpcPaging => {
                self.measurement = Measurement::EpcPaging;
                self.platform = HashSet::from([Platform::Sgx]);
                self.sort_data = false;
            },
            ExperimentType::CpuCyclesTuple => {
                self.measurement = Measurement::Throughput;
                self.dataset = HashSet::from([Dataset::CacheExceed, Dataset::CacheFit]);
                self.platform = HashSet::from([Platform::Sgx]);
                self.sort_data = false;
            },
            ExperimentType::Custom => (),
        }
    }
}

/// Commandline is a builder for a std::process::Command or its tokio equivalent.
/// The actual `std::process::Command` struct cannot be `Clone`, so this is needed to easily pass it around before actually running the command.
/// Sadly, I cannot include a method to create a `tokio::process::Command` from this, because including tokio in common is impossible: the frontend also uses the common crate, and you cannot use tokio in a webapp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commandline {
    pub app: Platform,
    pub args: Vec<String>,
}

impl Commandline {
    pub fn new(platform: Platform) -> Self {
        Self { app: platform, args: vec![] }
    }
    pub fn with_args(platform: Platform, args: &[&str]) -> Self {
        let args = args.iter().map(|a| a.to_string()).collect();
        Self { app: platform, args }
    }
    pub fn add_args<S: Display>(&mut self, name: &str, value: S) {
        self.args.push(name.to_string());
        self.args.push(value.to_string());
    }
    /// Adds all the values in `iter` as values of the last option of the `Commandline`s in `cmds` for each item in `cmds`.
    /// Example: `cmds` =  `["./app -a CHT"]` becomes `["./app -a CHT", "./app -a RHO"] if `iter` contains "RHO".
    /// Panics if `cmds` is empty.
    pub fn double_cmds_with_different_arg_value<S: Display, I: Iterator<Item = S>>(
        cmds: &mut Vec<Commandline>,
        iter: &mut I,
    ) {
        let l = cmds.len();
        for val in iter {
            let curr_l = cmds.len();
            cmds.extend_from_within(0..l);
            for cmd in cmds.iter_mut().skip(curr_l) {
                let d_arg = cmd.args.last_mut().unwrap();
                *d_arg = val.to_string();
            }
        }
    }
    pub fn app_string(&self) -> String {
        match self.app {
            Platform::Sgx => "./fake_teebench".to_string(),
            Platform::Native => "./fake_teebench".to_string(),
        }
    }
}

impl std::fmt::Display for Commandline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_joined = self.args.join(" ");
        write!(f, "{} {}", self.app_string(), args_joined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn dataset_to_from_string() {
        let ds_enum = Dataset::CacheExceed;
        let ds = ds_enum.to_string();
        assert_eq!(ds, "Cache Exceed");
        assert_eq!(Dataset::from_str(&ds).unwrap(), ds_enum);
    }

    #[test]
    fn profiling_configuration_to_teebench_cmd_default() {
        let c = ProfilingConfiguration::new(
            vec![Algorithm::Cht],
            ExperimentType::Custom,
            Parameter::Threads,
            Measurement::Throughput,
            2,
            2,
            2,
            vec![Dataset::CacheFit],
            vec![Platform::Sgx],
            false,
        );
        // let mut cmd = Commandline::new(&Platform::Sgx);
        // cmd.add_args("-a", "CHT");
        // cmd.add_args("-d", "Cache Fit");
        // cmd.add_args("-n", "2");
        let cmd = Commandline::with_args(
            Platform::Sgx,
            &vec!["-a", "CHT", "-d", "cache-fit", "-n", "2"],
        );
        for (to_be_tested, expected) in c.to_teebench_cmd().iter().zip(vec![cmd]) {
            assert_eq!(to_be_tested, &expected);
        }
    }

    #[test]
    fn profiling_configuration_to_teebench_cmd_multiple_cmds() {
        let c = ProfilingConfiguration::new(
            vec![Algorithm::Cht, Algorithm::Rho],
            ExperimentType::default(),
            Parameter::DataSkew,
            Measurement::default(),
            2,
            8,
            2,
            vec![Dataset::CacheExceed, Dataset::CacheFit],
            vec![Platform::Sgx, Platform::Native],
            true,
        );
        #[rustfmt::skip]
        let cmds = [
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8",],),
            Commandline::with_args(Platform::Sgx,   &vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8",],),
            Commandline::with_args(Platform::Native,&vec!["-a", "RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8",],),
        ];
        let to_be_tested = c.to_teebench_cmd();

        assert_eq!(to_be_tested.len(), cmds.len());
        for tested_cmd in to_be_tested {
            assert!(cmds.contains(&tested_cmd));
        }
    }
}
