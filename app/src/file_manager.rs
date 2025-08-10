use std::fs;
use std::path::{Path, PathBuf};

pub struct FileManager {
    current_dir: PathBuf,
    entries: Vec<PathBuf>,
    selected_index: usize,
    page_size: usize,
}

impl FileManager {
    pub fn new<P: AsRef<Path>>(path: P, page_size: usize) -> std::io::Result<Self> {
        let mut fm = FileManager {
            current_dir: path.as_ref().to_path_buf(),
            entries: Vec::new(),
            selected_index: 0,
            page_size,
        };
        fm.refresh_entries()?;
        Ok(fm)
    }

    pub fn back(&mut self) -> std::io::Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected_index = 0;
            self.refresh_entries()?;
        }

        Ok(())
    }

    pub fn enter(&mut self) -> std::io::Result<Option<PathBuf>> {
        if let Some(selected) = self.entries.get(self.selected_index) {
            if selected.is_dir() {
                self.current_dir = selected.clone();
                self.selected_index = 0;
                self.refresh_entries()?;
            } else {
                return Ok(Some(selected.to_owned()));
            }
        }

        Ok(None)
    }

    /// Move selection up by one, wrap to last if at first
    pub fn up(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.selected_index == 0 {
            self.selected_index = self.entries.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    /// Move selection down by one, wrap to first if at last
    pub fn down(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.selected_index = (self.selected_index + 1) % self.entries.len();
    }

    /// Move selection up by one page, wrapping around
    pub fn next_page(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.selected_index < self.page_size {
            // Wrap around to last page (last partial page included)
            let remainder = self.entries.len() % self.page_size;
            let last_page_start = if remainder == 0 {
                self.entries.len() - self.page_size
            } else {
                self.entries.len() - remainder
            };
            self.selected_index = last_page_start
                + (self.selected_index % self.page_size).min(remainder.saturating_sub(1));
        } else {
            self.selected_index -= self.page_size;
        }
    }

    /// Move selection down by one page, wrapping around
    pub fn prev_page(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let next = self.selected_index + self.page_size;
        if next >= self.entries.len() {
            // Wrap back to first page
            self.selected_index %= self.page_size;
        } else {
            self.selected_index = next;
        }
    }

    pub fn get_entries(&self) -> &[PathBuf] {
        &self.entries
    }

    pub fn get_page_entries(&self) -> &[PathBuf] {
        let page_start = (self.selected_index / self.page_size) * self.page_size;
        let page_end = usize::min(page_start + self.page_size, self.entries.len());
        &self.entries[page_start..page_end]
    }

    pub fn get_selected(&self) -> Option<&PathBuf> {
        self.entries.get(self.selected_index)
    }

    fn refresh_entries(&mut self) -> std::io::Result<()> {
        let mut entries: Vec<PathBuf> = fs::read_dir(&self.current_dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        self.entries = entries;

        if self.selected_index >= self.entries.len() {
            self.selected_index = 0;
        }

        Ok(())
    }
}
