use crate::app::AppCommand;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AppMenuItem {
    OpenRom,
    Exit,
}

impl fmt::Display for AppMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            AppMenuItem::OpenRom => "OPEN ROM",
            AppMenuItem::Exit => "EXIT",
        };
        write!(f, "{text}")
    }
}

pub struct AppMenu {
    items: Vec<AppMenuItem>,
    selected_index: usize,
}

impl Default for AppMenu {
    fn default() -> Self {
        Self {
            items: vec![AppMenuItem::OpenRom, AppMenuItem::Exit],
            selected_index: 0,
        }
    }
}

impl AppMenu {
    pub fn get_items(&self) -> Vec<String> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if i == self.selected_index {
                    format!(">{line}<")
                } else {
                    line.to_string()
                }
            })
            .collect()
    }

    pub fn move_up(&mut self) {
        self.selected_index = core::move_prev_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn move_down(&mut self) {
        self.selected_index = core::move_next_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn select(&self) -> Option<AppCommand> {
        let item = self.items[self.selected_index];

        match item {
            AppMenuItem::OpenRom => Some(AppCommand::PickFile),
            AppMenuItem::Exit => Some(AppCommand::Quit),
        }
    }
}
