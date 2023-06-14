use crate::data_types::{Algorithm, JobIdType, JobResult};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumVariantNames};
use time::OffsetDateTime;
use yewdux::prelude::Store;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompilationStatus {
    Uncompiled,
    Compiling,
    Successful(String),
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerfReportStatus {
    None,
    // TODO Add variant Waiting with job id. Then I could make the stop button a trash button (to trash the job, not the whole commit). And there wouldn't be so many spinners.
    Running(JobIdType),
    Successful,
    Failed,
}

pub type CommitIdType = i64;

/// A commit represents an algorithm/operator and its performance report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    /// What the user entered as the commit message.
    pub title: String,
    /// Version of the commit, eg. (v0.3).
    pub version: String,
    /// Type of the operator.
    pub operator: Operator,
    /// Time this operator was uploaded.
    pub datetime: OffsetDateTime,
    /// C or C++ code.
    pub code: String,
    /// Holds the finished Performance Report experiments.
    pub report: Option<JobResult>,
    /// Client-side-set ID of this commit, just gets incremented with each commit.
    pub id: CommitIdType,
    /// Compilation status
    pub compilation: CompilationStatus,
    /// Whether a PerfReport job is running right now for this commit
    pub perf_report_running: PerfReportStatus,
    /// Which other commit or Algorithm should serve as the baseline. Other commits are identified by Algorithm::Commit(CommitIdType).
    pub baseline: Algorithm,
}

impl Commit {
    pub fn new(
        title: String,
        version: String,
        operator: Operator,
        datetime: OffsetDateTime,
        code: String,
        reports: Option<JobResult>,
        id: CommitIdType,
        baseline: Algorithm,
    ) -> Self {
        Commit {
            title,
            version,
            operator,
            datetime,
            code,
            report: reports,
            id,
            compilation: CompilationStatus::Uncompiled,
            perf_report_running: PerfReportStatus::None,
            baseline,
        }
    }
    pub fn get_title(&self) -> String {
        format!("{}_v{}", self.title, self.version)
    }
    pub fn get_time_of_day(&self) -> String {
        let format = time::format_description::parse("[hour]:[minute]").unwrap();
        self.datetime.format(&format).unwrap()
    }
    pub fn get_date(&self) -> String {
        let format = time::format_description::parse("[day].[month]").unwrap();
        self.datetime.format(&format).unwrap()
    }
}

// TODO Would it be a good idea to put another field in here that encodes an error to communicate with the server? Depending on its value the commit list could display a field to reload the list.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct CommitState(pub Vec<Commit>);

impl CommitState {
    pub fn new(commits: Vec<Commit>) -> Self {
        Self(commits)
    }
    pub fn get_by_id(&self, id: &CommitIdType) -> Option<&Commit> {
        self.0.iter().find(|c| &c.id == id)
    }
    pub fn get_by_id_mut(&mut self, id: &CommitIdType) -> Option<&mut Commit> {
        self.0.iter_mut().find(|c| &c.id == id)
    }
    pub fn get_by_title(&self, title: &str) -> Vec<&Commit> {
        self.0.iter().filter(|c| c.title == title).collect()
    }
    pub fn get_by_name(&self, name: &str) -> Vec<&Commit> {
        let (title, version) = name.split_once("_v").unwrap();
        self.0
            .iter()
            .filter(|c| c.title == title && c.version == version)
            .collect()
    }
    pub fn get_latest(&self) -> Option<&Commit> {
        self.0.iter().max_by(|a, b| a.id.cmp(&b.id))
    }
    pub fn get_title(&self, id: &CommitIdType) -> Option<String> {
        self.0.iter().find(|c| &c.id == id).map(|c| c.get_title())
    }
    pub fn get_title_by_algorithm(&self, alg: &Algorithm) -> Option<String> {
        if let Algorithm::Commit(id) = alg {
            self.get_title(id)
        } else {
            Some(alg.to_string())
        }
    }
    pub fn push_commit(&mut self, c: Commit) {
        self.0.push(c);
    }
    pub fn get_diffs(&self) -> Vec<Option<String>> {
        use imara_diff::intern::InternedInput;
        use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
        let mut res = vec![None];
        for window in self.0.windows(2) {
            let old = window[0].code.as_str();
            let new = window[1].code.as_str();
            let input = InternedInput::new(old, new);
            let diff = diff(
                Algorithm::Histogram,
                &input,
                UnifiedDiffBuilder::new(&input),
            );
            res.push(Some(diff));
        }
        res
    }
}

use std::collections::{HashMap, HashSet};
impl CommitState {
    pub fn get_used_code(&self, algorithms: &HashSet<Algorithm>) -> HashMap<Algorithm, String> {
        let mut map = HashMap::new();
        for a in algorithms {
            if let Algorithm::Commit(id) = a {
                let c = self
                    .get_by_id(id)
                    .expect("Frontend might have sent a nonexistent commit id!");
                map.insert(*a, c.code.clone());
            }
        }
        map
    }
}

/// Holds the data from the upload form.
#[derive(Debug, Clone, PartialEq, Store)]
pub struct UploadCommitFormState {
    // TODO Strings might not need Options around them, just put an empty string there? The web also doesn't differentiate between nothing inputed yet and empty string, I think.
    pub title: Option<String>,
    // TODO If you feel fancy, you can add form validation for this field with the crate `lenient_semver`.
    pub version: Option<String>,
    pub operator: Option<Operator>,
    pub code: Option<String>,
    pub baseline: Option<Algorithm>,
}

impl Default for UploadCommitFormState {
    fn default() -> Self {
        Self {
            title: None,
            version: None,
            operator: Some(Operator::Join),
            code: None,
            baseline: Some(Algorithm::Rho),
        }
    }
}

impl UploadCommitFormState {
    // TODO Can this be converted to some From<Commit> implementation and use the automatic into?
    /// Only call after you verified that the form has been filled in correctly. Otherwise this panics
    pub fn to_commit(&self) -> Commit {
        let c;
        // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get current time").as_secs();
        let now = OffsetDateTime::now_utc();
        let timestamp = format!(
            "{:02}{:02}{:02}{:06}",
            now.hour(),
            now.minute(),
            now.second(),
            now.microsecond()
        )
        .parse::<CommitIdType>()
        .unwrap();
        c = Commit::new(
            self.title.clone().unwrap(),
            self.version.clone().unwrap(),
            self.operator.clone().unwrap(),
            OffsetDateTime::now_utc(),
            self.code.clone().unwrap(),
            None,
            timestamp,
            self.baseline.clone().unwrap(),
        );
        c
    }
    pub fn verify(&self) -> bool {
        self.title.is_some()
            && self.version.is_some()
            && self.operator.is_some()
            && self.code.is_some()
            && self.baseline.is_some()
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
