#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    pub title: String,
    pub code: String,
    pub report: Option<Report>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub performance_gain: u32,
}
