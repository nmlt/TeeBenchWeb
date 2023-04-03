use crate::data_types::{Algorithm, JobResult};
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

pub type CommitIdType = usize;

/// A commit represents an algorithm/operator and its performance report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    /// What the user entered as the commit message.
    pub title: String,
    /// Type of the operator.
    pub operator: Operator,
    /// Time this operator was uploaded.
    pub datetime: OffsetDateTime,
    /// C or C++ code.
    pub code: String,
    /// Holds the finished Performance Report experiments.
    pub reports: Option<JobResult>, // TODO Rename to report
    /// Client-side-set ID of this commit, just gets incremented with each commit.
    pub id: CommitIdType,
    /// Compilation status
    pub compilation: CompilationStatus,
    /// Whether a PerfReport job is running right now for this commit
    pub perf_report_running: bool,
    /// Which other commit or Algorithm should serve as the baseline. Other commits are identified by Algorithm::Commit(CommitIdType).
    pub baseline: Algorithm,
}

impl Commit {
    pub fn new(
        title: String,
        operator: Operator,
        datetime: OffsetDateTime,
        code: String,
        reports: Option<JobResult>,
        id: usize,
        baseline: Algorithm,
    ) -> Self {
        Commit {
            title,
            operator,
            datetime,
            code,
            reports,
            id,
            compilation: CompilationStatus::Uncompiled,
            perf_report_running: false,
            baseline,
        }
    }
}

// TODO Would it be a good idea to put another field in here that encodes an error to communicate with the server? Depending on its value the commit list could display a field to reload the list.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct CommitState(pub Vec<Commit>);

impl CommitState {
    pub fn new(commits: Vec<Commit>) -> Self {
        Self(commits)
    }
    pub fn get_id(&self, id: &CommitIdType) -> Option<&Commit> {
        self.0.iter().find(|c| &c.id == id)
    }
    pub fn get_id_mut(&mut self, id: &CommitIdType) -> Option<&mut Commit> {
        self.0.iter_mut().find(|c| &c.id == id)
    }
    pub fn get_title(&self, title: &str) -> Vec<&Commit> {
        self.0.iter().filter(|c| c.title == title).collect()
    }
    pub fn get_latest(&self) -> Option<&Commit> {
        self.0.iter().max_by(|a, b| a.id.cmp(&b.id))
    }
    pub fn push_commit(&mut self, c: Commit) {
        self.0.push(c);
    }
}
