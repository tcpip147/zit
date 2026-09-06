use std::{fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn new(identity_key: impl Into<String>) -> Result<Self, &'static str> {
        let identity_key = identity_key.into();
        if identity_key.trim().is_empty() {
            return Err("project identity must not be empty");
        }
        Ok(Self(identity_key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectAvailability {
    Available,
    Unavailable { message: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectMetadata {
    pub id: ProjectId,
    pub repository_path: PathBuf,
    pub display_name: String,
    pub availability: ProjectAvailability,
}

/// The lane and palette identity occupied by a commit in one history row.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommitGraphMarker {
    /// Zero-based horizontal lane.
    pub lane: usize,
    /// Stable palette index for this lane during one history load.
    pub color: usize,
}

/// One graph connection entering or leaving a commit history row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitGraphEdge {
    /// Full identity of the commit this active lane is waiting to reach.
    pub waiting_commit_id: String,
    /// Lane at the beginning of the half-row segment.
    pub from_lane: usize,
    /// Lane at the end of the half-row segment.
    pub to_lane: usize,
    /// Stable palette index carried by this edge.
    pub color: usize,
    /// Whether this edge connects to a commit rather than carrying through unchanged.
    pub tip: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitSummary {
    pub object_id: String,
    pub parent_ids: Vec<String>,
    pub summary: String,
    pub message_body: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub authored_at_seconds: i64,
    pub authored_at_offset_minutes: i32,
    pub commit_time_seconds: i64,
    pub commit_time_offset_minutes: i32,
    pub commit_lane: CommitGraphMarker,
    pub input_edges: Vec<CommitGraphEdge>,
    pub output_edges: Vec<CommitGraphEdge>,
    pub is_head: bool,
    pub branches: Vec<String>,
    pub remote_branches: Vec<String>,
    pub reachable_from_current: bool,
    pub reachable_from_local: bool,
    pub reachable_from_remote: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectData {
    pub project_id: ProjectId,
    pub head: Option<String>,
    pub commits: Vec<CommitSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectLoadState {
    NotLoaded,
    Loading { generation: u64 },
    Ready(ProjectData),
    Failed { generation: u64, message: String },
}
