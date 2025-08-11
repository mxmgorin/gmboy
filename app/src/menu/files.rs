use crate::app::AppCmd;
use crate::config::AppConfig;
use crate::file_browser::FileBrowser;
use crate::menu::MAX_ROMS_PER_PAGE;
use crate::PlatformFileSystem;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilesMenu {
    file_manager: FileBrowser,
}

#[derive(Debug, Clone)]
pub struct FileMenuItem {
    pub name: String,
    pub path: PathBuf,
}

impl FilesMenu {
    pub fn get_iterator(&self) -> impl Iterator<Item = String> + '_ {
        self.file_manager
            .get_page_entries()
            .iter()
            .enumerate()
            .map(move |(i, path)| {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if i == (self.file_manager.selected_index % self.file_manager.page_size) {
                    format!("◀{name}▶")
                } else {
                    name
                }
            })
    }

    pub fn move_up(&mut self) {
        self.file_manager.up();
    }

    pub fn move_down(&mut self) {
        self.file_manager.down();
    }

    pub fn move_left(&mut self) -> Option<AppCmd> {
        self.file_manager.prev_page();

        None
    }

    pub fn move_right(&mut self) -> Option<AppCmd> {
        self.file_manager.next_page();

        None
    }

    pub fn select(&mut self, _config: &AppConfig) -> (Option<AppCmd>, bool) {
        if let Some(selected) = self.file_manager.get_selected() {
            return if selected.is_dir() {
                self.file_manager.enter().unwrap();
                (None, false)
            } else {
                (Some(AppCmd::LoadFile(selected.clone())), false)
            };
        }

        (None, false)
    }

    pub fn next_page(&mut self) {
        self.file_manager.next_page();
    }

    pub fn prev_page(&mut self) {
        self.file_manager.prev_page();
    }

    pub fn new(_fs: &dyn PlatformFileSystem) -> Self {
        Self {
            file_manager: FileBrowser::new(".", MAX_ROMS_PER_PAGE).unwrap(),
        }
    }
}
