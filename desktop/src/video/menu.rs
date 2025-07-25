use crate::app::AppCommand;
use crate::config::AppConfig;
use std::collections::VecDeque;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum AppMenuItem {
    Resume,
    SaveState(usize),
    LoadState(usize),
    OpenRom,
    Options,
    Interface,
    Back,
    Exit,
    NextPalette,
    ToggleFps,
    ToggleFullscreen,
}

fn start() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::OpenRom,
        AppMenuItem::Options,
        AppMenuItem::Exit,
    ]
    .into_boxed_slice()
}

fn options() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::Interface, AppMenuItem::Back].into_boxed_slice()
}

fn interface() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::NextPalette,
        AppMenuItem::ToggleFullscreen,
        AppMenuItem::ToggleFps,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn pause() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Resume,
        AppMenuItem::SaveState(0),
        AppMenuItem::LoadState(0),
        AppMenuItem::OpenRom,
        AppMenuItem::Options,
        AppMenuItem::Exit,
    ]
    .into_boxed_slice()
}

pub struct AppMenu {
    prev_items: VecDeque<Box<[AppMenuItem]>>,
    items: Box<[AppMenuItem]>,
    selected_index: usize,
}

impl AppMenu {
    pub fn new(with_cart: bool) -> Self {
        Self {
            prev_items: VecDeque::with_capacity(4),
            items: if with_cart { pause() } else { start() },
            selected_index: 0,
        }
    }

    pub fn get_items(&self, config: &AppConfig) -> Box<[String]> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let line = line.to_string(config);
                if i == self.selected_index {
                    format!(">{line}<")
                } else {
                    line
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

    pub fn cancel(&mut self) {
        if let Some(prev) = self.prev_items.pop_back() {
            self.selected_index = 0;
            self.items = prev;
        }
    }

    pub fn select(&mut self) -> Option<AppCommand> {
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
            AppMenuItem::Options => {
                self.next_items(options());

                None
            }
            AppMenuItem::Interface => {
                self.next_items(interface());

                None
            }
            AppMenuItem::Back => {
                self.cancel();

                None
            }
            AppMenuItem::NextPalette => Some(AppCommand::NextPalette),
            AppMenuItem::ToggleFps => Some(AppCommand::ToggleFps),
            AppMenuItem::ToggleFullscreen => Some(AppCommand::ToggleFullscreen),
        }
    }

    fn next_items(&mut self, items: Box<[AppMenuItem]>) {
        let prev = mem::replace(&mut self.items, items);
        self.selected_index = 0;
        self.prev_items.push_front(prev);
    }
}

impl AppMenuItem {
    pub fn to_string(self, config: &AppConfig) -> String {
        match self {
            AppMenuItem::Resume => "Resume".to_string(),
            AppMenuItem::OpenRom => "Open Rom".to_string(),
            AppMenuItem::Exit => "Exit".to_string(),
            AppMenuItem::SaveState(i) => format!("Save ({i})"),
            AppMenuItem::LoadState(i) => format!("Load ({i})"),
            AppMenuItem::Options => "Options".to_string(),
            AppMenuItem::Interface => "Interface".to_string(),
            AppMenuItem::Back => "Back".to_string(),
            AppMenuItem::NextPalette => "Next Palette".to_string(),
            AppMenuItem::ToggleFps => format!("FPS{}", get_suffix(config.interface.show_fps)),
            AppMenuItem::ToggleFullscreen => {
                format!("Fullscreen{}", get_suffix(config.interface.is_fullscreen))
            }
        }
    }
}

fn get_suffix(enabled: bool) -> &'static str {
    if enabled {
        "(●)"
    } else {
        ""
    }
}
