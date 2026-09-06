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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitSummary {
    pub object_id: String,
    pub parent_ids: Vec<String>,
    pub summary: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_at_seconds: i64,
    pub authored_at_offset_minutes: i32,
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
