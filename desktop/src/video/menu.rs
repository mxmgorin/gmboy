use crate::app::AppCommand;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AppMenuItem {
    Resume,
    SaveState(usize),
    LoadState(usize),
    OpenRom,
    Exit,
}

fn start() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::OpenRom, AppMenuItem::Exit].into_boxed_slice()
}

fn pause() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Resume,
        AppMenuItem::SaveState(0),
        AppMenuItem::LoadState(0),
        AppMenuItem::OpenRom,
        AppMenuItem::Exit,
    ]
    .into_boxed_slice()
}

pub struct AppMenu {
    items: Box<[AppMenuItem]>,
    selected_index: usize,
}

impl AppMenu {
    pub fn new(with_cart: bool) -> Self {
        Self {
            items: if with_cart { pause() } else { start() },
            selected_index: 0,
        }
    }

    pub fn get_items(&self) -> Box<[String]> {
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

    pub fn move_right(&mut self) -> Option<AppCommand> {
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::SaveState(i) => {
                *i = core::move_next_wrapped(*i, 99);

                None
            }
            AppMenuItem::LoadState(i) => {
                *i = core::move_next_wrapped(*i, 99);

                None
            }
            _ => None,
        }
    }

    pub fn move_left(&mut self) -> Option<AppCommand> {
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::SaveState(i) => {
                *i = core::move_prev_wrapped(*i, 99);

                None
            }
            AppMenuItem::LoadState(i) => {
                *i = core::move_prev_wrapped(*i, 99);

                None
            }
            _ => None,
        }
    }

    pub fn select(&self) -> Option<AppCommand> {
        let item = self.items[self.selected_index];

        match item {
            AppMenuItem::Resume => Some(AppCommand::TogglePause),
            AppMenuItem::OpenRom => Some(AppCommand::PickFile),
            AppMenuItem::Exit => Some(AppCommand::Quit),
            AppMenuItem::SaveState(i) => Some(AppCommand::SaveState(
                core::emu::state::SaveStateCommand::Create,
                i,
            )),
            AppMenuItem::LoadState(i) => Some(AppCommand::SaveState(
                core::emu::state::SaveStateCommand::Load,
                i,
            )),
        }
    }
}

impl fmt::Display for AppMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppMenuItem::Resume => write!(f, "Resume"),
            AppMenuItem::OpenRom => write!(f, "Open Rom"),
            AppMenuItem::Exit => write!(f, "Exit"),
            AppMenuItem::SaveState(i) => write!(f, "Save ({i})"),
            AppMenuItem::LoadState(i) => write!(f, "Load ({i})"),
        }
    }
}
