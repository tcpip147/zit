use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};

use super::{
    GixRepositoryLoader, OpenProjectSession, ProjectAvailability, ProjectId, ProjectMetadata,
    ProjectPage, RepositoryLoader, SessionProject,
};

#[derive(Clone, Debug)]
pub enum ProjectStoreEvent {
    Restored {
        project_ids: Vec<ProjectId>,
        active: Option<ProjectId>,
    },
    MetadataChanged(ProjectId),
    ActiveChanged(Option<ProjectId>),
    ProjectRemoved {
        project_id: ProjectId,
        active: Option<ProjectId>,
    },
}

pub struct ProjectStore {
    projects: Vec<ProjectMetadata>,
    pages: HashMap<ProjectId, Entity<ProjectPage>>,
    active: Option<ProjectId>,
    session_path: PathBuf,
    loader: Arc<dyn RepositoryLoader>,
}

pub struct GlobalProjectStore(pub Entity<ProjectStore>);
impl Global for GlobalProjectStore {}

impl ProjectStore {
    pub fn create_repository(
        &mut self,
        parent: PathBuf,
        name: String,
        cx: &mut Context<Self>,
    ) -> Result<ProjectId, String> {
        let path = std::fs::canonicalize(&parent).map_err(|_| "Folder does not exist.".to_owned())?;
        let name = name.trim();
        if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
            return Err("Enter a valid project name.".to_owned());
        }
        if path.join(".git").exists() {
            return Err("The selected folder is already a Git repository.".to_owned());
        }
        let repository = match gix::init(&path) {
            Ok(repository) => repository,
            Err(error) => {
                return Err(format!("Could not create Git repository: {error}"));
            }
        };
        let metadata = self.loader.inspect(repository.workdir().unwrap_or(&path), Some(name));
        let id = metadata.id.clone();
        if self.projects.iter().any(|project| project.id == id) {
            self.activate(&id, cx);
            return Ok(id);
        }
        let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));
        self.projects.push(metadata);
        self.pages.insert(id.clone(), page.clone());
        self.active = Some(id.clone());
        page.update(cx, |page, cx| page.activate(cx));
        self.persist();
        cx.emit(ProjectStoreEvent::Restored {
            project_ids: self.projects.iter().map(|p| p.id.clone()).collect(),
            active: self.active.clone(),
        });
        cx.notify();
        Ok(id)
    }
    pub fn add_repository(
        &mut self,
        path: PathBuf,
        display_name: String,
        cx: &mut Context<Self>,
    ) -> Result<ProjectId, String> {
        let path = std::fs::canonicalize(&path).map_err(|_| "Folder does not exist.".to_owned())?;
        if !path.join(".git").exists() {
            return Err("The selected folder is not a Git repository.".to_owned());
        }
        if let Some(existing) = self
            .projects
            .iter()
            .find(|p| std::fs::canonicalize(&p.repository_path).ok().as_ref() == Some(&path))
        {
            let id = existing.id.clone();
            self.activate(&id, cx);
            return Ok(id);
        }
        let metadata = self.loader.inspect(&path, Some(&display_name));
        let id = metadata.id.clone();
        let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));
        self.projects.push(metadata);
        self.pages.insert(id.clone(), page);
        self.active = Some(id.clone());
        self.persist();
        cx.emit(ProjectStoreEvent::Restored {
            project_ids: self.projects.iter().map(|p| p.id.clone()).collect(),
            active: self.active.clone(),
        });
        cx.notify();
        Ok(id)
    }
    pub fn new(session_path: PathBuf) -> Self {
        Self::with_loader(session_path, Arc::new(GixRepositoryLoader))
    }

    pub fn with_loader(session_path: PathBuf, loader: Arc<dyn RepositoryLoader>) -> Self {
        Self {
            projects: Vec::new(),
            pages: HashMap::new(),
            active: None,
            session_path,
            loader,
        }
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalProjectStore>().0.clone()
    }
    pub fn projects(&self) -> &[ProjectMetadata] {
        &self.projects
    }
    pub fn active_project(&self) -> Option<&ProjectId> {
        self.active.as_ref()
    }
    pub fn page(&self, id: &ProjectId) -> Option<Entity<ProjectPage>> {
        self.pages.get(id).cloned()
    }

    pub fn restore(&mut self, cx: &mut Context<Self>) {
        let session = match OpenProjectSession::load(&self.session_path) {
            Ok(session) => session,
            Err(error) => {
                eprintln!("Zit ignored project session: {error}");
                OpenProjectSession::default()
            }
        };
        self.projects.clear();
        self.pages.clear();
        let mut seen = HashSet::new();
        for saved in session.projects {
            let mut metadata = self
                .loader
                .inspect(&saved.repository_path, Some(&saved.display_name));
            if matches!(
                metadata.availability,
                ProjectAvailability::Unavailable { .. }
            ) && let Ok(id) = ProjectId::new(saved.identity_key.clone())
            {
                metadata.id = id;
            }
            if !seen.insert(metadata.id.clone()) {
                continue;
            }
            let id = metadata.id.clone();
            let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));
            self.projects.push(metadata);
            self.pages.insert(id, page);
        }
        self.active = session
            .active_project
            .filter(|id| self.pages.contains_key(id))
            .or_else(|| {
                self.projects
                    .iter()
                    .find(|project| matches!(project.availability, ProjectAvailability::Available))
                    .or_else(|| self.projects.first())
                    .map(|project| project.id.clone())
            });
        let project_ids = self
            .projects
            .iter()
            .map(|project| project.id.clone())
            .collect();
        cx.emit(ProjectStoreEvent::Restored {
            project_ids,
            active: self.active.clone(),
        });
        cx.notify();
    }

    /// Creates local repositories used to exercise the project-page UI when no
    /// real session has been configured yet.
    pub fn seed_test_projects(&mut self, count: usize, cx: &mut Context<Self>) {
        if !self.projects.is_empty() || count == 0 {
            return;
        }
        let root = self
            .session_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("test-projects");
        for number in 1..=count {
            let path = if number == 2 {
                std::path::PathBuf::from(r"D:\Project\zed")
            } else {
                root.join(format!("project-{number}"))
            };
            if let Err(error) = std::fs::create_dir_all(&path) {
                eprintln!("Zit could not create test project {number}: {error}");
                continue;
            }
            if !path.join(".git").exists()
                && let Err(error) = gix::init(&path)
            {
                eprintln!("Zit could not initialize test project {number}: {error}");
                continue;
            }
            let mut metadata = self.loader.inspect(&path, None);
            metadata.display_name = format!("Test Project {number}");
            if self
                .projects
                .iter()
                .any(|project| project.id == metadata.id)
            {
                continue;
            }
            let id = metadata.id.clone();
            let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));
            self.projects.push(metadata);
            self.pages.insert(id, page);
        }
        self.active = self.projects.first().map(|project| project.id.clone());
        self.persist();
        cx.emit(ProjectStoreEvent::Restored {
            project_ids: self
                .projects
                .iter()
                .map(|project| project.id.clone())
                .collect(),
            active: self.active.clone(),
        });
        cx.notify();
    }

    pub fn set_test_project_repository(
        &mut self,
        index: usize,
        repository_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(existing) = self.projects.get(index).cloned() else {
            return false;
        };
        let mut metadata = self
            .loader
            .inspect(&repository_path, Some(&existing.display_name));
        metadata.display_name = existing.display_name;
        let new_id = metadata.id.clone();
        let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));
        self.pages.remove(&existing.id);
        self.pages.insert(new_id.clone(), page);
        self.projects[index] = metadata;
        if self.active.as_ref() == Some(&existing.id) {
            self.active = Some(new_id);
        }
        self.persist();
        cx.notify();
        true
    }

    pub fn set_active(&mut self, id: Option<ProjectId>, cx: &mut Context<Self>) -> bool {
        let id = id.filter(|id| self.pages.contains_key(id));
        if self.active == id {
            return false;
        }
        self.active = id;
        self.persist();
        cx.emit(ProjectStoreEvent::ActiveChanged(self.active.clone()));
        cx.notify();
        true
    }

    /// Replaces the first seeded project with a real repository while keeping
    /// the remaining test tabs intact.
    pub fn set_first_test_project(
        &mut self,
        repository_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(first) = self.projects.first() else {
            return false;
        };
        let already_configured = std::fs::canonicalize(&first.repository_path).ok()
            == std::fs::canonicalize(&repository_path).ok();
        let is_seeded_project_one = first.repository_path.file_name().is_some_and(|name| {
            name.eq_ignore_ascii_case("project-1")
                && first
                    .repository_path
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .is_some_and(|parent| parent.eq_ignore_ascii_case("test-projects"))
        });
        if first.display_name != "Test Project 1"
            && first.display_name != "Project 1"
            && !already_configured
            && !is_seeded_project_one
        {
            return false;
        }

        let old_id = first.id.clone();
        let mut metadata = self.loader.inspect(&repository_path, Some("Project 1"));
        metadata.display_name = "Project 1".into();
        let new_id = metadata.id.clone();
        let page = cx.new(|_| ProjectPage::new(metadata.clone(), self.loader.clone()));

        self.projects[0] = metadata;
        self.pages.remove(&old_id);
        self.pages.insert(new_id.clone(), page);
        if self.active.as_ref() == Some(&old_id) {
            self.active = Some(new_id);
        }
        self.persist();
        cx.emit(ProjectStoreEvent::Restored {
            project_ids: self
                .projects
                .iter()
                .map(|project| project.id.clone())
                .collect(),
            active: self.active.clone(),
        });
        cx.notify();
        true
    }

    pub fn activate(&mut self, id: &ProjectId, cx: &mut Context<Self>) -> bool {
        let Some(page) = self.pages.get(id).cloned() else {
            return false;
        };
        self.set_active(Some(id.clone()), cx);
        page.update(cx, |page, cx| page.activate(cx));
        true
    }

    pub fn remove(&mut self, id: &ProjectId, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.projects.iter().position(|project| &project.id == id) else {
            return false;
        };
        self.projects.remove(index);
        if let Some(page) = self.pages.remove(id) {
            page.update(cx, |page, cx| page.cancel_pending_load(cx));
        }
        if self.active.as_ref() == Some(id) {
            self.active = self
                .projects
                .get(index)
                .or_else(|| {
                    index
                        .checked_sub(1)
                        .and_then(|index| self.projects.get(index))
                })
                .map(|project| project.id.clone());
        }
        self.persist();
        cx.emit(ProjectStoreEvent::ProjectRemoved {
            project_id: id.clone(),
            active: self.active.clone(),
        });
        cx.notify();
        true
    }

    pub fn update_metadata(&mut self, metadata: ProjectMetadata, cx: &mut Context<Self>) -> bool {
        let Some(project) = self
            .projects
            .iter_mut()
            .find(|project| project.id == metadata.id)
        else {
            return false;
        };
        *project = metadata.clone();
        if let Some(page) = self.pages.get(&metadata.id) {
            page.update(cx, |page, cx| page.update_metadata(metadata.clone(), cx));
        }
        self.persist();
        cx.emit(ProjectStoreEvent::MetadataChanged(metadata.id));
        cx.notify();
        true
    }

    pub fn refresh_metadata(&mut self, id: &ProjectId, cx: &mut Context<Self>) -> bool {
        let Some(existing) = self
            .projects
            .iter()
            .find(|project| &project.id == id)
            .cloned()
        else {
            return false;
        };
        let mut metadata = self
            .loader
            .inspect(&existing.repository_path, Some(&existing.display_name));
        metadata.id = id.clone();
        self.update_metadata(metadata, cx)
    }

    fn persist(&self) {
        let session = OpenProjectSession {
            version: super::session::SESSION_VERSION,
            projects: self
                .projects
                .iter()
                .map(|project| SessionProject {
                    identity_key: project.id.as_str().to_owned(),
                    repository_path: project.repository_path.clone(),
                    display_name: project.display_name.clone(),
                })
                .collect(),
            active_project: self.active.clone(),
        };
        if let Err(error) = session.save(&self.session_path) {
            eprintln!("Zit could not save project session: {error}");
        }
    }
}

impl EventEmitter<ProjectStoreEvent> for ProjectStore {}
