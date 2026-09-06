use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
    path::Path,
};

use gix::{ObjectId, revision::walk::Sorting, traverse::commit::simple::CommitTimeOrder};

use super::{
    CommitGraphEdge, CommitGraphMarker, CommitSummary, ProjectAvailability, ProjectData, ProjectId,
    ProjectMetadata,
};

const HISTORY_LIMIT: usize = 100;

#[derive(Debug)]
struct BranchTip {
    name: String,
    oid: ObjectId,
}

#[derive(Default)]
struct GraphColorPool(usize);

impl GraphColorPool {
    fn next(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

/// Applies the project history lane algorithm to commits that are already in display order.
///
/// Existing graph fields are replaced. Repository I/O and presentation are deliberately kept
/// outside this function so topology can be tested independently.
pub fn build_commit_graph(commits: &mut [CommitSummary]) {
    let mut previous_outputs: Vec<CommitGraphEdge> = Vec::new();
    let mut colors = GraphColorPool::default();

    for commit in commits {
        let mut marker = CommitGraphMarker::default();
        let mut inputs = Vec::new();

        if !previous_outputs.is_empty() {
            let mut reached = false;
            let mut occupied = 0;
            let mut seen_destinations = HashSet::new();

            for edge in previous_outputs
                .iter()
                .filter(|edge| seen_destinations.insert(edge.to_lane))
            {
                if edge.waiting_commit_id == commit.object_id {
                    if !reached {
                        reached = true;
                        marker.lane = inputs.len();
                        marker.color = edge.color;
                        occupied += 1;
                    }
                    inputs.push(CommitGraphEdge {
                        waiting_commit_id: commit.parent_ids.first().cloned().unwrap_or_default(),
                        from_lane: edge.to_lane,
                        to_lane: marker.lane,
                        color: edge.color,
                        tip: true,
                    });
                } else {
                    inputs.push(CommitGraphEdge {
                        waiting_commit_id: edge.waiting_commit_id.clone(),
                        from_lane: edge.to_lane,
                        to_lane: occupied,
                        color: edge.color,
                        tip: false,
                    });
                    occupied += 1;
                }
            }

            if !reached {
                marker.lane = inputs.len();
                marker.color = colors.next();
            }
        }

        let mut outputs = inputs
            .iter()
            .filter(|edge| !edge.tip)
            .map(|edge| CommitGraphEdge {
                waiting_commit_id: edge.waiting_commit_id.clone(),
                from_lane: edge.to_lane,
                to_lane: edge.to_lane,
                color: edge.color,
                tip: false,
            })
            .collect::<Vec<_>>();

        let waiting_lanes: HashMap<&str, &CommitGraphEdge> = inputs
            .iter()
            .map(|edge| (edge.waiting_commit_id.as_str(), edge))
            .collect();

        for (parent_index, parent_id) in commit.parent_ids.iter().enumerate() {
            if parent_index == 0 {
                outputs.push(CommitGraphEdge {
                    waiting_commit_id: parent_id.clone(),
                    from_lane: marker.lane,
                    to_lane: marker.lane,
                    color: marker.color,
                    tip: true,
                });
            } else if let Some(edge) = waiting_lanes.get(parent_id.as_str()) {
                outputs.push(CommitGraphEdge {
                    waiting_commit_id: parent_id.clone(),
                    from_lane: marker.lane,
                    to_lane: edge.to_lane,
                    color: edge.color,
                    tip: true,
                });
            } else {
                outputs.push(CommitGraphEdge {
                    waiting_commit_id: parent_id.clone(),
                    from_lane: marker.lane,
                    to_lane: outputs.len(),
                    color: colors.next(),
                    tip: true,
                });
            }
        }

        outputs.sort_by_key(|edge| edge.to_lane);
        commit.commit_lane = marker;
        commit.input_edges = inputs;
        commit.output_edges = outputs.clone();
        previous_outputs = outputs;
    }
}

#[derive(Clone, Debug)]
pub struct RepositoryError {
    message: String,
}

impl RepositoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    pub fn user_message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
impl std::error::Error for RepositoryError {}

pub trait RepositoryLoader: Send + Sync + 'static {
    fn inspect(&self, path: &Path, fallback_name: Option<&str>) -> ProjectMetadata;
    fn load_history(&self, metadata: &ProjectMetadata) -> Result<ProjectData, RepositoryError>;
}

#[derive(Default)]
pub struct GixRepositoryLoader;

impl GixRepositoryLoader {
    fn normalized_key(path: &Path) -> String {
        let value = path.to_string_lossy().replace('\\', "/");
        if cfg!(windows) {
            value.to_lowercase()
        } else {
            value
        }
    }

    fn fallback_metadata(
        path: &Path,
        fallback_name: Option<&str>,
        message: String,
    ) -> ProjectMetadata {
        let absolute = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let key = Self::normalized_key(&absolute);
        let id = ProjectId::new(if key.is_empty() {
            path.to_string_lossy().into_owned()
        } else {
            key
        })
        .expect("repository path creates a non-empty identity");
        let display_name = fallback_name
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .or_else(|| {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "Unavailable project".into());
        ProjectMetadata {
            id,
            repository_path: path.to_path_buf(),
            display_name,
            availability: ProjectAvailability::Unavailable { message },
        }
    }
}

impl RepositoryLoader for GixRepositoryLoader {
    fn inspect(&self, path: &Path, fallback_name: Option<&str>) -> ProjectMetadata {
        match gix::discover(path) {
            Ok(repository) => {
                let git_dir = fs::canonicalize(repository.git_dir())
                    .unwrap_or_else(|_| repository.git_dir().to_path_buf());
                let id = ProjectId::new(Self::normalized_key(&git_dir))
                    .expect("canonical Git directory is non-empty");
                let repository_path = repository
                    .workdir()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| path.to_path_buf());
                let display_name = repository_path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .filter(|name| !name.is_empty())
                    .or_else(|| fallback_name.map(str::to_owned))
                    .unwrap_or_else(|| "Git repository".into());
                ProjectMetadata {
                    id,
                    repository_path,
                    display_name,
                    availability: ProjectAvailability::Available,
                }
            }
            Err(error) => Self::fallback_metadata(
                path,
                fallback_name,
                format!("Repository is unavailable: {error}"),
            ),
        }
    }

    fn load_history(&self, metadata: &ProjectMetadata) -> Result<ProjectData, RepositoryError> {
        if let ProjectAvailability::Unavailable { message } = &metadata.availability {
            return Err(RepositoryError::new(message.clone()));
        }
        let mut repository = gix::discover(&metadata.repository_path)
            .map_err(|error| RepositoryError::new(format!("Could not open repository: {error}")))?;
        repository.object_cache_size(16 * 1024 * 1024);
        let head = repository.head_id().ok().map(|id| id.detach());
        let head_branch = repository
            .head()
            .ok()
            .and_then(|head| head.referent_name().map(|name| name.shorten().to_string()));

        let references = repository.references().map_err(|error| {
            RepositoryError::new(format!("Could not read repository references: {error}"))
        })?;
        let local = references.local_branches().map_err(|error| {
            RepositoryError::new(format!("Could not read local branches: {error}"))
        })?;
        let references = repository.references().map_err(|error| {
            RepositoryError::new(format!("Could not read repository references: {error}"))
        })?;
        let remote = references.remote_branches().map_err(|error| {
            RepositoryError::new(format!("Could not read remote branches: {error}"))
        })?;
        let branch_tips = local
            .chain(remote)
            .filter_map(|reference| {
                let mut reference = reference.ok()?;
                let name = reference.name().shorten().to_string();
                let oid = reference.peel_to_commit().ok()?.id;
                Some(BranchTip { name, oid })
            })
            .collect::<Vec<_>>();

        if branch_tips.is_empty() {
            return Ok(ProjectData {
                project_id: metadata.id.clone(),
                head: head.map(|id| id.to_string()),
                commits: Vec::new(),
            });
        }

        let mut branch_map: HashMap<ObjectId, Vec<String>> = HashMap::new();
        for branch in &branch_tips {
            branch_map
                .entry(branch.oid)
                .or_default()
                .push(branch.name.clone());
        }
        if let (Some(head), Some(head_branch)) = (head, head_branch.as_deref())
            && let Some(branches) = branch_map.get_mut(&head)
            && let Some(index) = branches.iter().position(|branch| branch == head_branch)
        {
            let head_branch = branches.remove(index);
            branches.insert(0, head_branch);
        }

        let walk = repository
            .rev_walk(branch_tips.iter().map(|branch| branch.oid))
            .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
            .all()
            .map_err(|error| RepositoryError::new(format!("Could not read history: {error}")))?;
        let mut commits = Vec::new();
        for info in walk.take(HISTORY_LIMIT) {
            let info = info.map_err(|error| {
                RepositoryError::new(format!("Could not traverse history: {error}"))
            })?;
            let commit = repository.find_commit(info.id).map_err(|error| {
                RepositoryError::new(format!("Could not read a commit: {error}"))
            })?;
            let author = commit.author().map_err(|error| {
                RepositoryError::new(format!("Could not read a commit author: {error}"))
            })?;
            let time = author.time().map_err(|error| {
                RepositoryError::new(format!("Could not read a commit date: {error}"))
            })?;
            let commit_time = commit.time().map_err(|error| {
                RepositoryError::new(format!("Could not read a commit time: {error}"))
            })?;
            let message = commit.message().map_err(|error| {
                RepositoryError::new(format!("Could not read a commit message: {error}"))
            })?;
            let summary = message.summary().to_string();
            let message_body = message
                .body()
                .map(|body| body.to_string())
                .unwrap_or_default();
            let parent_ids = commit.parent_ids().map(|id| id.to_string()).collect();
            let object_id = info.id.to_string();
            commits.push(CommitSummary {
                object_id,
                parent_ids,
                summary,
                message_body,
                author_name: author.name.to_string(),
                author_email: author.email.to_string(),
                authored_at_seconds: time.seconds,
                authored_at_offset_minutes: time.offset,
                commit_time_seconds: commit_time.seconds,
                commit_time_offset_minutes: commit_time.offset,
                commit_lane: CommitGraphMarker::default(),
                input_edges: Vec::new(),
                output_edges: Vec::new(),
                is_head: head.is_some_and(|head| head == info.id),
                branches: branch_map.get(&info.id).cloned().unwrap_or_default(),
            });
        }
        build_commit_graph(&mut commits);
        Ok(ProjectData {
            project_id: metadata.id.clone(),
            head: head.map(|id| id.to_string()),
            commits,
        })
    }
}
