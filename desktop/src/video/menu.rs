use crate::app::{AppCommand, ChangeAppConfigCmd};
use crate::config::AppConfig;
use std::collections::VecDeque;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum AppMenuItem {
    Resume,
    SaveState(usize),
    LoadState(usize),
    OpenRom,
    OptionsMenu,
    InterfaceMenu,
    Back,
    Exit,
    Palette,
    ToggleFps,
    ToggleFullscreen,
    AudioMenu,
    Volume,
    Scale,
    DeveloperMenu,
    TileWindow,
    SpinDuration,
    SystemMenu,
    SaveStateOnExit,
    NormalSpeed,
    TurboSpeed,
    SlowSpeed,
    RewindSize,
    RewindInterval,
    AudioBufferSize,
    MuteTurbo,
    MuteSlow,
    DefaultConfig,
    RestartGame,
}

fn system_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::SaveStateOnExit,
        AppMenuItem::NormalSpeed,
        AppMenuItem::TurboSpeed,
        AppMenuItem::SlowSpeed,
        AppMenuItem::RewindSize,
        AppMenuItem::RewindInterval,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn developer_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::TileWindow,
        AppMenuItem::SpinDuration,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn start_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::OpenRom,
        AppMenuItem::OptionsMenu,
        AppMenuItem::Exit,
    ]
    .into_boxed_slice()
}

fn options_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::InterfaceMenu,
        AppMenuItem::AudioMenu,
        AppMenuItem::SystemMenu,
        AppMenuItem::DeveloperMenu,
        AppMenuItem::DefaultConfig,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn interface_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Palette,
        AppMenuItem::ToggleFullscreen,
        AppMenuItem::ToggleFps,
        AppMenuItem::Scale,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn game_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Resume,
        AppMenuItem::SaveState(0),
        AppMenuItem::LoadState(0),
        AppMenuItem::RestartGame,
        AppMenuItem::OpenRom,
        AppMenuItem::OptionsMenu,
        AppMenuItem::Exit,
    ]
    .into_boxed_slice()
}

fn audio_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Volume,
        AppMenuItem::AudioBufferSize,
        AppMenuItem::MuteTurbo,
        AppMenuItem::MuteSlow,
        AppMenuItem::Back,
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
            items: if with_cart { game_menu() } else { start_menu() },
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
                    format!("◀{line}▶")
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
            AppMenuItem::Scale => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Scale(1.0))),
            AppMenuItem::SpinDuration => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SpinDuration(1),
            )),
            AppMenuItem::Volume => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Volume(0.05))),
            AppMenuItem::ToggleFps => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
            }
            AppMenuItem::TileWindow => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::TileWindow))
            }
            AppMenuItem::SaveStateOnExit => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SaveStateOnExit,
            )),
            AppMenuItem::Palette => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::NextPalette)),
            AppMenuItem::Resume
            | AppMenuItem::OpenRom
            | AppMenuItem::OptionsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Exit
            | AppMenuItem::AudioMenu
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::SystemMenu => None,
            AppMenuItem::NormalSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::NormalSpeed(0.1),
            )),
            AppMenuItem::TurboSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::TurboSpeed(0.1),
            )),
            AppMenuItem::SlowSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SlowSpeed(0.1),
            )),
            AppMenuItem::RewindSize => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::RewindSize(25)))
            }
            AppMenuItem::RewindInterval => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::RewindInterval(1_000_000),
            )),
            AppMenuItem::AudioBufferSize => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::AudioBufferSize(2),
            )),
            AppMenuItem::MuteTurbo => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => None,
            AppMenuItem::RestartGame => None,
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
            AppMenuItem::Scale => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0))),
            AppMenuItem::SpinDuration => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SpinDuration(-1),
            )),
            AppMenuItem::Volume => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)))
            }
            AppMenuItem::ToggleFps => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
            }
            AppMenuItem::TileWindow => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::TileWindow))
            }
            AppMenuItem::SaveStateOnExit => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SaveStateOnExit,
            )),
            AppMenuItem::Palette => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::PrevPalette)),
            AppMenuItem::Resume
            | AppMenuItem::OpenRom
            | AppMenuItem::OptionsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Exit
            | AppMenuItem::AudioMenu
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::SystemMenu => None,
            AppMenuItem::NormalSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::NormalSpeed(-0.1),
            )),
            AppMenuItem::TurboSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::TurboSpeed(-0.1),
            )),
            AppMenuItem::SlowSpeed => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SlowSpeed(-0.1),
            )),
            AppMenuItem::RewindSize => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::RewindSize(-25),
            )),
            AppMenuItem::RewindInterval => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::RewindInterval(-1_000_000),
            )),
            AppMenuItem::AudioBufferSize => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::AudioBufferSize(-2),
            )),
            AppMenuItem::MuteTurbo => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => None,
            AppMenuItem::RestartGame => None,
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
                core::emu::state::SaveStateCmd::Create,
                i,
            )),
            AppMenuItem::LoadState(i) => Some(AppCommand::SaveState(
                core::emu::state::SaveStateCmd::Load,
                i,
            )),
            AppMenuItem::OptionsMenu => {
                self.next_items(options_menu());

                None
            }
            AppMenuItem::InterfaceMenu => {
                self.next_items(interface_menu());

                None
            }
            AppMenuItem::Back => {
                self.cancel();

                None
            }
            AppMenuItem::Palette => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::NextPalette)),
            AppMenuItem::ToggleFps => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
            }
            AppMenuItem::AudioMenu => {
                self.next_items(audio_menu());

                None
            }
            AppMenuItem::Volume | AppMenuItem::Scale => None,
            AppMenuItem::DeveloperMenu => {
                self.next_items(developer_menu());

                None
            }
            AppMenuItem::TileWindow => {
                Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::TileWindow))
            }
            AppMenuItem::SpinDuration => None,
            AppMenuItem::SystemMenu => {
                self.next_items(system_menu());

                None
            }
            AppMenuItem::SaveStateOnExit => Some(AppCommand::ChangeConfig(
                ChangeAppConfigCmd::SaveStateOnExit,
            )),
            AppMenuItem::NormalSpeed => None,
            AppMenuItem::TurboSpeed => None,
            AppMenuItem::SlowSpeed => None,
            AppMenuItem::RewindSize => None,
            AppMenuItem::RewindInterval => None,
            AppMenuItem::AudioBufferSize => None,
            AppMenuItem::MuteTurbo => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => Some(AppCommand::ChangeConfig(ChangeAppConfigCmd::Default)),
            AppMenuItem::RestartGame => Some(AppCommand::RestartGame),
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
            AppMenuItem::SaveState(i) => format!("Save({i})"),
            AppMenuItem::LoadState(i) => format!("Load({i})"),
            AppMenuItem::OptionsMenu => "Options".to_string(),
            AppMenuItem::InterfaceMenu => "Interface".to_string(),
            AppMenuItem::Back => "Back".to_string(),
            AppMenuItem::Palette => format!("Palette({})", config.interface.selected_palette_idx),
            AppMenuItem::ToggleFps => format!("FPS{}", get_suffix(config.interface.show_fps)),
            AppMenuItem::ToggleFullscreen => {
                format!("Fullscreen{}", get_suffix(config.interface.is_fullscreen))
            }
            AppMenuItem::AudioMenu => "Audio".to_string(),
            AppMenuItem::Volume => {
                format!("Volume({})", (config.audio.volume * 100.0) as i32)
            }
            AppMenuItem::Scale => {
                format!("Scale({})", config.interface.scale)
            }
            AppMenuItem::DeveloperMenu => "Developer".to_string(),
            AppMenuItem::TileWindow => {
                format!("Tile Window{}", get_suffix(config.interface.tile_window))
            }
            AppMenuItem::SpinDuration => {
                format!("Spin({})", config.get_emu_config().spin_duration.as_nanos())
            }
            AppMenuItem::SystemMenu => "System".to_string(),
            AppMenuItem::SaveStateOnExit => format!(
                "Save State On Exit{}",
                get_suffix(config.save_state_on_exit)
            ),
            AppMenuItem::NormalSpeed => {
                format!("Normal Speed(x{})", config.emulation.normal_speed)
            }
            AppMenuItem::TurboSpeed => {
                format!("Turbo Speed(x{})", config.emulation.turbo_speed)
            }
            AppMenuItem::SlowSpeed => {
                format!("Slow Speed(x{})", config.emulation.slow_speed)
            }
            AppMenuItem::RewindSize => format!("Rewind Size({})", config.emulation.rewind_size),
            AppMenuItem::RewindInterval => format!(
                "Rewind Interval({})",
                config.emulation.rewind_interval.as_secs()
            ),
            AppMenuItem::AudioBufferSize => format!("Buffer Size({})", config.audio.buffer_size),
            AppMenuItem::MuteTurbo => format!("Mute Turbo{}", get_suffix(config.audio.mute_turbo)),
            AppMenuItem::MuteSlow => format!("Mute Slow{}", get_suffix(config.audio.mute_slow)),
            AppMenuItem::DefaultConfig => "Reset Default".to_string(),
            AppMenuItem::RestartGame => "Restart".to_string(),
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
