use std::{
    env, fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::ProjectId;

pub const SESSION_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionProject {
    pub identity_key: String,
    pub repository_path: PathBuf,
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenProjectSession {
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<SessionProject>,
    #[serde(default)]
    pub active_project: Option<ProjectId>,
}

impl Default for OpenProjectSession {
    fn default() -> Self {
        Self {
            version: SESSION_VERSION,
            projects: Vec::new(),
            active_project: None,
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    Io(std::io::Error),
    Json(serde_json::Error),
    UnsupportedVersion(u32),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "could not access project session: {error}"),
            Self::Json(error) => write!(f, "invalid project session: {error}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported project session version {version}")
            }
        }
    }
}

impl std::error::Error for SessionError {}

impl OpenProjectSession {
    pub fn load(path: &Path) -> Result<Self, SessionError> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => return Err(SessionError::Io(error)),
        };
        let session: Self = serde_json::from_str(&content).map_err(SessionError::Json)?;
        if session.version != SESSION_VERSION {
            return Err(SessionError::UnsupportedVersion(session.version));
        }
        Ok(session)
    }

    pub fn save(&self, path: &Path) -> Result<(), SessionError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(SessionError::Io)?;
        }
        let temporary = path.with_extension("json.tmp");
        let bytes = serde_json::to_vec_pretty(self).map_err(SessionError::Json)?;
        let mut file = fs::File::create(&temporary).map_err(SessionError::Io)?;
        file.write_all(&bytes).map_err(SessionError::Io)?;
        file.sync_all().map_err(SessionError::Io)?;
        if !path.exists() {
            return fs::rename(temporary, path).map_err(SessionError::Io);
        }
        let backup = path.with_extension("json.bak");
        if backup.exists() {
            fs::remove_file(&backup).map_err(SessionError::Io)?;
        }
        fs::rename(path, &backup).map_err(SessionError::Io)?;
        match fs::rename(&temporary, path) {
            Ok(()) => {
                let _ = fs::remove_file(backup);
                Ok(())
            }
            Err(error) => {
                let _ = fs::rename(backup, path);
                Err(SessionError::Io(error))
            }
        }
    }
}

pub fn default_session_path() -> PathBuf {
    if let Some(path) = env::var_os("ZIT_SESSION") {
        return PathBuf::from(path);
    }
    #[cfg(target_os = "windows")]
    let base = env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let base =
        env::var_os("HOME").map(|home| PathBuf::from(home).join("Library/Application Support"));
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")));
    base.unwrap_or_else(env::temp_dir)
        .join("zit")
        .join("session.json")
}
