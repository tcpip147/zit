use std::{fmt, fs, path::Path};

use super::{CommitSummary, ProjectAvailability, ProjectData, ProjectId, ProjectMetadata};

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
        let repository = gix::discover(&metadata.repository_path)
            .map_err(|error| RepositoryError::new(format!("Could not open repository: {error}")))?;
        let head = match repository.head_id() {
            Ok(head) => head.detach(),
            Err(_) => {
                return Ok(ProjectData {
                    project_id: metadata.id.clone(),
                    head: None,
                    commits: Vec::new(),
                });
            }
        };
        let walk = repository
            .rev_walk([head])
            .all()
            .map_err(|error| RepositoryError::new(format!("Could not read history: {error}")))?;
        let mut commits = Vec::new();
        for info in walk {
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
            let summary = commit
                .message()
                .map_err(|error| {
                    RepositoryError::new(format!("Could not read a commit message: {error}"))
                })?
                .summary()
                .to_string();
            commits.push(CommitSummary {
                object_id: info.id.to_string(),
                summary,
                author_name: author.name.to_string(),
                author_email: author.email.to_string(),
                authored_at_seconds: time.seconds,
                authored_at_offset_minutes: time.offset,
            });
        }
        Ok(ProjectData {
            project_id: metadata.id.clone(),
            head: Some(head.to_string()),
            commits,
        })
    }
}
