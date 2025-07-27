use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RomsLibrary {
    last_path: Option<PathBuf>,
    recent_paths: HashSet<PathBuf>,
}

impl RomsLibrary {
    pub fn add(&mut self, path: PathBuf) {
        self.recent_paths.insert(path.clone());
        self.last_path = Some(path);
    }

    pub fn remove(&mut self, path: &Path) {
        self.recent_paths.remove(path);

        if self.last_path.as_deref() == Some(path) {
            self.last_path = None;
        }
    }

    pub fn get_last_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_path.as_ref()?);

        Some(path.file_stem()?.to_string_lossy())
    }

    pub fn get_last_path(&self) -> Option<&PathBuf> {
        self.last_path.as_ref()
    }

    pub fn get_or_create() -> Self {
        let path = Self::get_path();

        if path.exists() {
            let res = core::read_json_file::<RomsLibrary>(&path);
            let Ok(lib) = res else {
                return Default::default();
            };

            lib
        } else {
            Default::default()
        }
    }

    pub fn get(&self) -> &HashSet<PathBuf> {
        &self.recent_paths
    }

    pub fn get_path() -> PathBuf {
        let exe_dir = core::get_exe_path();

        exe_dir.join("library.json")
    }
}
