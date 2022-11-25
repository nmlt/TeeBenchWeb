use chrono::{DateTime, offset::Utc};

// TODO Add an ID that the server generates to uniquely identify a commit, indenpendently of the user supplied title.
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    pub title: String,
    pub datetime: DateTime<Utc>,
    pub code: String,
    pub report: Option<Report>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub performance_gain: u32,
}
