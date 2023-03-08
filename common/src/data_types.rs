use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use indoc::writedoc;
pub use strum::VariantNames;
use strum_macros::{Display, EnumIter, EnumString, EnumVariantNames};
use thiserror::Error;
use yewdux::prelude::Store;

use std::collections::HashSet;
use std::fmt::Display;

/// Error type for Experiments.
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TeeBenchWebError {
    #[error("Could not retrieve results")]
    NoResults,
    #[error("Unknown error")]
    #[default]
    Unknown,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default, EnumString, Display, EnumVariantNames,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Operator {
    #[default]
    Join,
    #[strum(to_string = "GROUP BY")]
    GroupBy,
    Projection,
    #[strum(to_string = "ORDER BY")]
    OrderBy,
}

/// A commit represents an algorithm/operator and its performance report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub title: String,
    /// Type of the operator.
    pub operator: Operator,
    /// Time this operator was uploaded.
    pub datetime: OffsetDateTime,
    /// C or C++ code.
    pub code: String,
    /// Holds the finished Performance Report experiments. Baseline is first stored in a hook, and after the reports are done, in the reports.
    pub reports: Vec<Report>, // TODO Change to JobResult in case there is an error.
    /// Top level findings (diplayed above all the charts in the performance report).
    pub findings: Vec<Finding>,
    // TODO Add an ID that the server generates to uniquely identify a commit, independently of the user supplied title.
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            title: String::default(),
            operator: Operator::default(),
            datetime: OffsetDateTime::now_utc(),
            code: String::default(),
            reports: Vec::default(),
            findings: Vec::default(),
        }
    }
}

impl Commit {
    pub fn new(
        title: String,
        operator: Operator,
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
            findings: vec![
                Finding::new("Performance Difference", "+ 3.6 %", FindingStyle::Good),
                Finding::new("Phase 1: Partition", "180/191 (+0)", FindingStyle::SoSo),
                Finding::new("Phase 2: Join", "11/191 (-4)", FindingStyle::Good),
                Finding::new("EPC Paging", "- 0.4 %", FindingStyle::Good),
            ],
        }
    }
}

/// One report can be transformed to one chart with findings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
// To make ProfilingConfiguration an enum depending on ExperimentType is a bad idea maybe, because then we'd have to match in every dispatch callback modifying the config. So instead we continue using the ExperimentType in the ProfilingConfig.
pub struct Report {
    pub config: ProfilingConfiguration,
    pub results: Vec<(TeebenchArgs, ExperimentResult)>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub matches: u64,
    #[serde(rename = "phaseBuildCycles")]
    pub phase_build_cycles: u64,
    #[serde(rename = "phasePartitionCycles")]
    pub phase_partition_cycles: u64,
    #[serde(rename = "phaseProbeCycles")]
    pub phase_probe_cycles: u64,
    #[serde(rename = "cyclesPerTuple")]
    pub cycles_per_tuple: u64,
    #[serde(rename = "timePartitionUsec")]
    pub time_partition_usec: u64,
    #[serde(rename = "timeJoinUsec")]
    pub time_join_usec: u64,
    #[serde(rename = "timeTotalUsec")]
    pub time_total_usec: u64,
    pub throughput: f64,
    //pub epc_paging: u64,
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

pub type JobResult = Result<Report, TeeBenchWebError>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum JobStatus {
    #[default]
    Waiting,
    Done {
        runtime: Duration,
        result: JobResult,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub config: ProfilingConfiguration,
    pub submitted: OffsetDateTime,
    pub status: JobStatus,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            config: ProfilingConfiguration::default(),
            submitted: OffsetDateTime::now_utc(),
            status: JobStatus::default(),
        }
    }
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
    PartialOrd,
    Ord,
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
    Rho,
    // #[strum(to_string = "JOIN - PHT")]
    Pht,
    // #[strum(to_string = "JOIN - PSM")]
    Psm,
    // #[strum(to_string = "JOIN - MWAY")]
    Mway,
    // #[strum(to_string = "JOIN - RHT")]
    Rht,
    // #[strum(to_string = "JOIN - RHO")]
    Cht,
    // #[strum(to_string = "JOIN - RSM")]
    Rsm,
    // #[strum(to_string = "JOIN - INL")]
    Inl,
    // #[strum(to_string = "JOIN - CRKJ")]
    Crkj,
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
    #[strum(to_string = "EPC Paging")]
    EpcPaging,
    Throughput,
    // #[strum(to_string = "CPU Cycles/Tuple")]
    // CpuCyclesTuple,
    #[default]
    Custom,
}

impl ExperimentType {
    pub fn description(&self) -> &str {
        match self {
            Self::EpcPaging => {
                "View the first selected algorithm/operator's throughput and EPC misses in relation to the dataset size"
            }
            Self::Throughput => {
                "Compares througput of all selected algorithms for the selected dataset"
            }
            // Self::CpuCyclesTuple => {

            // }
            Self::Custom => {
                "A custom experiment"
            }
        }
    }
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
    PartialOrd,
    Ord,
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
    pub fn from_cmd_arg(string: &str) -> Result<Self, &'static str> {
        match string {
            "cache-fit" | "Cache Fit" | "CacheFit" => Ok(Dataset::CacheFit),
            "cache-exceed" | "Cache Exceed" | "CacheExceed" => Ok(Dataset::CacheExceed),
            _ => Err("Dataset can only be Cache Fit or Cache Exceed!"), //panic!("Dataset can only be Cache Fit or Cache Exceed!"),
        }
    }
}

// TODO Remove strum's EnumString and Display and use this instead. But it doesn't work...
// impl From<String> for Dataset {
//     fn from(string: String) -> Self {
//         //Dataset::try_from(string).unwrap()
//         Dataset::CacheFit
//     }
// }
// impl TryFrom<String> for Dataset {
//     type Error = &'static str;

//     fn try_from(string: String) -> std::result::Result<Self, Self::Error> {
//         match string.as_str() {
//             "cache-fit" | "Cache Fit" | "CacheFit" => Ok(Dataset::CacheFit),
//             "cache-exceed" | "Cache Exceed" | "CacheExceed" => Ok(Dataset::CacheExceed),
//             _ => Err("Dataset can only be Cache Fit or Cache Exceed!"),
//         }
//     }
// }

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Default,
    PartialOrd,
    Ord,
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

impl Platform {
    pub fn from_app_name(s: &str) -> Result<Self, String> {
        match s {
            "fake_teebench" | "sgx" | "app" => Ok(Self::Sgx),
            "native" => Ok(Self::Native),
            name => {
                let msg = format!("Platform cannot be {name}");
                Err(msg)
            }
        }
    }
    pub fn to_app_name(&self) -> String {
        match self {
            Self::Sgx => "./sgx".to_string(),
            Self::Native => "./native".to_string(),
        }
    }
    pub fn arg0_to_platform() -> Platform {
        // A program always has a name (right?).
        let arg0: String = std::env::args().next().unwrap();
        let p = PathBuf::from(arg0);
        // Impossible for the file name not to be valid utf-8, we just converted it from a (valid utf-8) String.
        let name = p.file_name().unwrap().to_str().unwrap();
        let Ok(pl) = Platform::from_app_name(name) else {
            // TODO Fix this. But ignore for now.
            return Platform::Sgx;
        };
        pl
    }
}

pub type StepType = i64;

/// Represents either a Profiling Experiment or a Performance Report Experiment.
///
/// - Depending on the `experiment_type` it's either a predefined experiment (those in the performance report, but also choosable in the profiling menu, although then it won't appear in the performance report tab) or a custom profiling experiment (those appear only in the profiling tab under in the job results view).
/// - For predefined experiments, many fields are not relevant:
///     - But the `algorithm` field always defines the current operator/algorithm and its baseline.
///     - `dataset` and `platform` might be sometimes relevant: `ExperimentType::Throughput` allows one dataset to be choosen.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Store)]
pub struct ProfilingConfiguration {
    pub algorithm: HashSet<Algorithm>, // TODO Rename to plural.
    pub experiment_type: ExperimentType,
    pub parameter: Parameter,
    pub measurement: Measurement,
    pub min: StepType,
    pub max: StepType,
    pub step: StepType,
    pub dataset: HashSet<Dataset>,   // TODO Rename to plural.
    pub platform: HashSet<Platform>, // TODO Rename to plural.
    pub sort_data: bool,
}

impl ProfilingConfiguration {
    pub fn new(
        algorithm: Vec<Algorithm>,
        experiment_type: ExperimentType,
        parameter: Parameter,
        measurement: Measurement,
        min: StepType,
        max: StepType,
        step: StepType,
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
            max: 8,
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
                Measurement: {}
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
            self.measurement,
            self.dataset,
            self.platform,
            self.sort_data
        )
    }
}

impl ProfilingConfiguration {
    pub fn param_value_iter(&self) -> std::iter::StepBy<std::ops::RangeInclusive<StepType>> {
        (self.min..=self.max).step_by(self.step as usize)
    }
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
        Commandline::double_cmds_with_different_arg_value(
            &mut res,
            &mut dataset_iter.map(|ds| ds.to_cmd_arg()),
        );
        let mut value_iter = self.param_value_iter(); // TODO Verify that these values form a valid range.
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

        for cmd in &mut res {
            cmd.add_flag("--csv");
        }
        res
    }
    /// Set default values for preconfigured experiment types. These then overwrite the values previously entered in the config form (if called in the frontend/profiling.rs in a dispatch). Not useful for now, as the config form can not represent the preconfigured experiments.
    // TODO This actually does not work for the <select> elements (at least it doesn't display the change. The store value changes).
    pub fn set_preconfigured_experiment(&mut self) {
        match self.experiment_type {
            ExperimentType::Throughput => {
                // self.parameter = Parameter::Threads;
                self.measurement = Measurement::Throughput;
                // self.min = 2;
                // self.max = 2;
                // self.step = 1;
                // self.dataset = HashSet::from([Dataset::CacheExceed, Dataset::CacheFit]);
                // self.platform = HashSet::from([Platform::Sgx]);
                // self.sort_data = false;
            }
            ExperimentType::EpcPaging => {
                self.measurement = Measurement::EpcPaging;
                // self.platform = HashSet::from([Platform::Sgx]);
                // self.sort_data = false;
            }
            // ExperimentType::CpuCyclesTuple => {
            //     self.measurement = Measurement::Throughput;
            //     // self.dataset = HashSet::from([Dataset::CacheExceed, Dataset::CacheFit]);
            //     // self.platform = HashSet::from([Platform::Sgx]);
            //     // self.sort_data = false;
            // }
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
        Self {
            app: platform,
            args: vec![],
        }
    }
    pub fn with_args(platform: Platform, args: &[&str]) -> Self {
        let args = args.iter().map(|a| a.to_string()).collect();
        Self {
            app: platform,
            args,
        }
    }
    pub fn add_args<S: Display>(&mut self, name: &str, value: S) {
        self.args.push(name.to_string());
        self.args.push(value.to_string());
    }
    pub fn add_flag(&mut self, name: &str) {
        self.args.push(name.to_string());
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
    pub fn to_teebench_args(&self) -> TeebenchArgs {
        let app_name = vec![self.app.to_app_name()];
        let iter = app_name.iter().chain(self.args.iter());
        let mut args = TeebenchArgs::from_iter_safe(iter).unwrap();
        args.app_name = self.app;
        args
    }
}

impl std::fmt::Display for Commandline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_joined = self.args.join(" ");
        write!(f, "{} {}", self.app.to_app_name(), args_joined)
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(
    Debug, Clone, StructOpt, PartialOrd, Ord, PartialEq, Eq, Hash, Default, Deserialize, Serialize,
)]
#[structopt(
    name = "TeeBench",
    about = "fake placeholder for testing that outputs teebench output. Because I don't have SGX on my dev machine."
)]
pub struct TeebenchArgs {
    /// The name of the application. Used to determine whether it is simulating Sgx or native.
    #[structopt(skip = Platform::arg0_to_platform())]
    pub app_name: Platform,
    ///`-d` - name of pre-defined dataset. Currently working: `cache-fit`, `cache-exceed`. Default: `cache-fit`
    #[structopt(short = "d", long, parse(try_from_str = Dataset::from_cmd_arg), default_value = "cache-fit")]
    pub dataset: Dataset,
    ///`-a` - join algorithm name. Currently working: see `common::data_types::Algorithm`.
    #[structopt(short = "a", long, default_value = "RHO")]
    pub algorithm: Algorithm,
    ///`-n` - number of threads used to execute the join algorithm. Default: `2`
    #[structopt(short = "n", long, default_value = "2")]
    pub threads: u8,
    ///`-l` - join selectivity. Should be a number between 0 and 100. Default: `100`
    #[structopt(short = "l", long, default_value = "100")]
    pub selectivity: u8,
    ///`-z` - data skew. Default: `0`
    #[structopt(short = "z", long, default_value = "0")]
    pub data_skew: u32,
    ///`-c` - seal chunk size in kBs. if set to 0 then seal everything at once. Default: `0`
    #[structopt(short = "c", long, default_value = "0")]
    pub seal_chunk_size: u32,
    ///`-r` - number of tuples of R relation. Default: `2097152`
    #[structopt(short = "r", long, default_value = "2097152")]
    pub r_tuples: u32,
    ///`-s` - number of tuples of S relation. Default: `2097152`
    #[structopt(short = "s", long, default_value = "2097152")]
    pub s_tuples: u32,
    ///`-t | --r-path` - filepath to build R relation. Default: `none`
    #[structopt(short = "t", long)]
    pub r_path: Option<String>,
    ///`-u | --s-path` - filepath to build S relation. Default `none`
    #[structopt(short = "u", long)]
    pub s_path: Option<String>,
    ///`-x` - size of R in MBs. Default: `none`
    #[structopt(short = "x", long)]
    pub r_size: Option<u32>,
    ///`-y` - size of S in MBs. Default: `none`
    #[structopt(short = "y", long)]
    pub s_size: Option<u32>,
    ///`--seal` - flag to seal join input data. Default: `false`
    #[structopt(long = "seal")]
    pub seal: bool,
    ///`--sort-r` - flag to pre-sort relation R. Default: `false`
    #[structopt(long = "sort-r")]
    pub sort_r: bool,
    ///`--sort-s` - flag to pre-sort relation S. Default: `false`
    #[structopt(long = "sort-s")]
    pub sort_s: bool,
    ///Change output to only print out data in csv format
    #[structopt(long)]
    pub csv: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn serde_json_report() {
        let mut report_struct = Report {
            config: ProfilingConfiguration::default(),
            results: vec![],
            findings: vec![],
        };
        report_struct
            .results
            .push((TeebenchArgs::default(), ExperimentResult::default()));
        let _report_json = serde_json::to_string(&report_struct).unwrap();
    }

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
            &vec!["-a", "CHT", "-d", "cache-fit", "-n", "2", "--csv"],
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
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","2","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","4","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","6","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-exceed","-z","8","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","CHT","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Sgx   ,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
            Commandline::with_args(Platform::Native,&vec!["-a","RHO","--sort-r","--sort-s","-d","cache-fit"   ,"-z","8","--csv",],),
        ];
        let to_be_tested = c.to_teebench_cmd();

        assert_eq!(to_be_tested.len(), cmds.len());
        for tested_cmd in to_be_tested {
            assert!(cmds.contains(&tested_cmd));
        }
    }
}
