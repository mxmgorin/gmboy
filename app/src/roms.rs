use crate::{get_base_dir, AppFilesystem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RomsList {
    last_path: Option<PathBuf>,
    recent_paths: HashSet<PathBuf>,
}

impl RomsList {
    /// Loads all `.gb` and `.gbc` files from the given directory.
    pub fn load_from_dir<P: AsRef<Path>>(
        &mut self,
        dir: P,
        filesystem: &Box<dyn AppFilesystem>,
    ) -> Result<usize, String> {
        let dir_path = dir.as_ref();
        self.recent_paths.clear();
        let files = filesystem.read_dir(dir_path)?;

        for file in files {
            let path = PathBuf::from(file);
            if let Some(name) = filesystem.get_file_name(&path) {
                if name.ends_with(".gb") || name.ends_with(".gbc") {
                    self.recent_paths.insert(path);
                }
            }
        }

        Ok(self.recent_paths.len())
    }

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

    pub fn get_last_path(&self) -> Option<&PathBuf> {
        self.last_path.as_ref()
    }

    pub fn get_or_create() -> Self {
        let path = Self::get_path();

        if path.exists() {
            let res: Result<RomsList, _> = core::read_json_file(&path);
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
        get_base_dir().join("roms.json")
    }
}
