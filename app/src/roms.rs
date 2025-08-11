use crate::{get_base_dir, PlatformFileSystem};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RomsState {
    pub last_browse_dir_path: Option<PathBuf>,
    pub selected_dir_path: Option<PathBuf>,
    opened_rom_paths: IndexSet<PathBuf>,
    loaded_rom_files: HashSet<String>,
}

impl RomsState {
    /// Loads all `.gb` and `.gbc` files from the given directory.
    pub fn load_from_dir<P: AsRef<Path>>(
        &mut self,
        dir: P,
        filesystem: &impl PlatformFileSystem,
    ) -> Result<usize, String> {
        let dir_path = dir.as_ref();
        self.loaded_rom_files.clear();
        let files = filesystem.read_dir(dir_path)?;
        self.selected_dir_path = Some(dir_path.to_owned());

        for file in files {
            let path = PathBuf::from(file);
            if let Some(name) = filesystem.get_file_name(&path) {
                if name.ends_with(".gb") || name.ends_with(".gbc") {
                    self.loaded_rom_files.insert(name);
                }
            }
        }

        Ok(self.loaded_rom_files.len())
    }

    pub fn on_opened(&mut self, path: PathBuf) {
        self.opened_rom_paths.insert(path.clone());
    }

    pub fn remove(&mut self, path: &Path) {
        self.opened_rom_paths.shift_remove(path);
    }

    pub fn get_last_path(&self) -> Option<&PathBuf> {
        self.opened_rom_paths.iter().last()
    }

    pub fn get_or_create(fs: &impl PlatformFileSystem) -> Self {
        let path = Self::get_path();

        let mut obj = if path.exists() {
            let res: Result<RomsState, _> = core::read_json_file(&path);
            let Ok(lib) = res else {
                return Default::default();
            };

            lib
        } else {
            Default::default()
        };

        if let Some(path) = obj.selected_dir_path.take() {
            if let Err(err) = obj.load_from_dir(path, fs) {
                log::error!("Failed load_from_dir: {err}");
            }
        }

        obj
    }

    /// Returns an iterator over the full paths of loaded ROM files.
    pub fn iter_loaded(&self) -> Option<impl Iterator<Item = PathBuf> + '_> {
        self.selected_dir_path.as_ref().map(|dir| {
            self.loaded_rom_files
                .iter()
                .map(move |file_name| dir.join(file_name))
        })
    }

    pub fn iter_opened(&self) -> impl Iterator<Item = &PathBuf> + '_ {
        self.opened_rom_paths.iter()
    }

    pub fn save_file(&self) {
        if let Err(err) = core::save_json_file(RomsState::get_path(), self) {
            log::error!("Failed to save ROMs: {err}");
        }
    }

    fn get_path() -> PathBuf {
        get_base_dir().join("roms.json")
    }
}
