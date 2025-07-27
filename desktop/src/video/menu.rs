use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::config::AppConfig;
use crate::video::frame_blend::{
    AdditiveFrameBlend, ExponentialFrameBlend, FrameBlendMode, GammaCorrectedFrameBlend,
    LinearFrameBlend,
};
use std::collections::VecDeque;
use std::mem;
use crate::video::game_window::LcdProfile;

#[derive(Debug, Clone, Copy)]
pub enum AppMenuItem {
    Resume,
    SaveState,
    LoadState,
    OpenRom,
    OptionsMenu,
    InterfaceMenu,
    Back,
    Quit,
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
    AutoSaveState,
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
    InputMenu,
    ComboInterval,
    PaletteInverted,
    FrameBlendMode,
    FrameBlendAlpha,
    FrameBlendFade,
    FrameBlendDim,
    VideoMenu,
}

fn video_menu(frame_blend_type: &FrameBlendMode) -> Box<[AppMenuItem]> {
    let mut items = Vec::with_capacity(5);
    items.push(AppMenuItem::FrameBlendMode);
    
    match frame_blend_type {
        FrameBlendMode::None => {}
        FrameBlendMode::Linear(_) => {
            items.push(AppMenuItem::FrameBlendAlpha);
            items.push(AppMenuItem::FrameBlendDim);
        }
        FrameBlendMode::Exponential(_) => {
            items.push(AppMenuItem::FrameBlendFade);
            items.push(AppMenuItem::FrameBlendDim);
        }
        FrameBlendMode::GammaCorrected(_) | FrameBlendMode::Additive(_) => {
            items.push(AppMenuItem::FrameBlendFade);
            items.push(AppMenuItem::FrameBlendAlpha);
            items.push(AppMenuItem::FrameBlendDim);
        }
        FrameBlendMode::Accurate(_) => {}
    }

    items.push(AppMenuItem::Back);

    items.into_boxed_slice()
}

fn input_menu() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::ComboInterval, AppMenuItem::Back].into_boxed_slice()
}

fn system_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::AutoSaveState,
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
        AppMenuItem::Quit,
    ]
    .into_boxed_slice()
}

fn options_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::InterfaceMenu,
        AppMenuItem::VideoMenu,
        AppMenuItem::AudioMenu,
        AppMenuItem::InputMenu,
        AppMenuItem::SystemMenu,
        AppMenuItem::DeveloperMenu,
        AppMenuItem::DefaultConfig,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn interface_menu() -> Box<[AppMenuItem]> {
    let mut items = Vec::with_capacity(6);
    items.push(AppMenuItem::Palette);
    items.push(AppMenuItem::PaletteInverted);
    items.push(AppMenuItem::ToggleFullscreen);
    items.push(AppMenuItem::ToggleFps);
    items.push(AppMenuItem::Scale);
    items.push(AppMenuItem::Back);

    items.into_boxed_slice()
}

fn game_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Resume,
        AppMenuItem::SaveState,
        AppMenuItem::LoadState,
        AppMenuItem::RestartGame,
        AppMenuItem::OpenRom,
        AppMenuItem::OptionsMenu,
        AppMenuItem::Quit,
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

    pub fn move_right(&mut self, config: &AppConfig) -> Option<AppCmd> {
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::SaveState => {
                let i = core::move_next_wrapped(config.current_save_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SaveIndex(i)))
            }
            AppMenuItem::LoadState => {
                let i = core::move_next_wrapped(config.current_load_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::LoadIndex(i)))
            }
            AppMenuItem::Scale => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0))),
            AppMenuItem::SpinDuration => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SpinDuration(1)))
            }
            AppMenuItem::Volume => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05))),
            AppMenuItem::ToggleFps => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
            }
            AppMenuItem::TileWindow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::TileWindow)),
            AppMenuItem::AutoSaveState => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::AutoSaveState))
            }
            AppMenuItem::Palette => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette)),
            AppMenuItem::Resume
            | AppMenuItem::OpenRom
            | AppMenuItem::OptionsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Quit
            | AppMenuItem::AudioMenu
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::SystemMenu => None,
            AppMenuItem::NormalSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NormalSpeed(0.1)))
            }
            AppMenuItem::TurboSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::TurboSpeed(0.1)))
            }
            AppMenuItem::SlowSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SlowSpeed(0.1)))
            }
            AppMenuItem::RewindSize => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::RewindSize(25)))
            }
            AppMenuItem::RewindInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::RewindInterval(1_000_000),
            )),
            AppMenuItem::AudioBufferSize => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::AudioBufferSize(2)))
            }
            AppMenuItem::MuteTurbo => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => None,
            AppMenuItem::RestartGame => None,
            AppMenuItem::InputMenu => None,
            AppMenuItem::ComboInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::ComboInterval(5_000),
            )),
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PaletteInverted))
            }
            AppMenuItem::FrameBlendMode => {
                let blend_type = match config.video.frame_blend_mode {
                    FrameBlendMode::None => FrameBlendMode::Linear(LinearFrameBlend::default()),
                    FrameBlendMode::Linear(_) => {
                        FrameBlendMode::Additive(AdditiveFrameBlend::default())
                    }
                    FrameBlendMode::Additive(_) => {
                        FrameBlendMode::Exponential(ExponentialFrameBlend::default())
                    }
                    FrameBlendMode::Exponential(_) => {
                        FrameBlendMode::GammaCorrected(GammaCorrectedFrameBlend::default())
                    }
                    FrameBlendMode::GammaCorrected(_) => FrameBlendMode::Accurate(LcdProfile::DMG),
                    FrameBlendMode::Accurate(_) => FrameBlendMode::None
                };
                self.items = video_menu(&blend_type);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    blend_type,
                )))
            }
            AppMenuItem::FrameBlendAlpha => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_alpha(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::FrameBlendFade => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_fade(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::FrameBlendDim => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_dim(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::VideoMenu => None,
        }
    }

    pub fn move_left(&mut self, config: &AppConfig) -> Option<AppCmd> {
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::SaveState => {
                let i = core::move_prev_wrapped(config.current_save_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SaveIndex(i)))
            }
            AppMenuItem::LoadState => {
                let i = core::move_prev_wrapped(config.current_load_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::LoadIndex(i)))
            }
            AppMenuItem::Scale => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0))),
            AppMenuItem::SpinDuration => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SpinDuration(-1)))
            }
            AppMenuItem::Volume => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05))),
            AppMenuItem::ToggleFps => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
            }
            AppMenuItem::TileWindow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::TileWindow)),
            AppMenuItem::AutoSaveState => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::AutoSaveState))
            }
            AppMenuItem::Palette => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevPalette)),
            AppMenuItem::Resume
            | AppMenuItem::OpenRom
            | AppMenuItem::OptionsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Quit
            | AppMenuItem::AudioMenu
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::SystemMenu => None,
            AppMenuItem::NormalSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NormalSpeed(-0.1)))
            }
            AppMenuItem::TurboSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::TurboSpeed(-0.1)))
            }
            AppMenuItem::SlowSpeed => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SlowSpeed(-0.1)))
            }
            AppMenuItem::RewindSize => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::RewindSize(-25)))
            }
            AppMenuItem::RewindInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::RewindInterval(-1_000_000),
            )),
            AppMenuItem::AudioBufferSize => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::AudioBufferSize(-2),
            )),
            AppMenuItem::MuteTurbo => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => None,
            AppMenuItem::RestartGame => None,
            AppMenuItem::InputMenu => None,
            AppMenuItem::ComboInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::ComboInterval(-5_000),
            )),
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PaletteInverted))
            }
            AppMenuItem::FrameBlendAlpha => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_alpha(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::FrameBlendMode => {
                let blend_type = match config.video.frame_blend_mode {
                    FrameBlendMode::None => {
                        FrameBlendMode::Accurate(LcdProfile::DMG)
                    }
                    FrameBlendMode::Linear(_) => {
                        FrameBlendMode::None
                    }
                    FrameBlendMode::Additive(_) => {
                        FrameBlendMode::Linear(LinearFrameBlend::default())
                    }
                    FrameBlendMode::Exponential(_) => {
                        FrameBlendMode::Additive(AdditiveFrameBlend::default())
                    }
                    FrameBlendMode::GammaCorrected(_) => FrameBlendMode::Exponential(ExponentialFrameBlend::default()),
                    FrameBlendMode::Accurate(_) => FrameBlendMode::GammaCorrected(GammaCorrectedFrameBlend::default())
                };

                self.items = video_menu(&blend_type);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    blend_type,
                )))
            }
            AppMenuItem::FrameBlendFade => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_fade(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::FrameBlendDim => {
                let mut mode = config.video.frame_blend_mode.clone();
                mode.change_dim(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameBlendMode(
                    mode,
                )))
            }
            AppMenuItem::VideoMenu => None,
        }
    }

    pub fn cancel(&mut self) {
        if let Some(prev) = self.prev_items.pop_back() {
            self.selected_index = 0;
            self.items = prev;
        }
    }

    pub fn select(&mut self, config: &AppConfig) -> Option<AppCmd> {
        let item = self.items[self.selected_index];

        match item {
            AppMenuItem::Resume => Some(AppCmd::TogglePause),
            AppMenuItem::OpenRom => Some(AppCmd::PickFile),
            AppMenuItem::Quit => Some(AppCmd::Quit),
            AppMenuItem::SaveState => Some(AppCmd::SaveState(
                core::emu::state::SaveStateCmd::Create,
                config.current_save_index,
            )),
            AppMenuItem::LoadState => Some(AppCmd::SaveState(
                core::emu::state::SaveStateCmd::Load,
                config.current_load_index,
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
            AppMenuItem::Palette => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette)),
            AppMenuItem::ToggleFps => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fps)),
            AppMenuItem::ToggleFullscreen => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Fullscreen))
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
            AppMenuItem::TileWindow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::TileWindow)),
            AppMenuItem::SpinDuration => None,
            AppMenuItem::SystemMenu => {
                self.next_items(system_menu());

                None
            }
            AppMenuItem::AutoSaveState => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::AutoSaveState))
            }
            AppMenuItem::NormalSpeed => None,
            AppMenuItem::TurboSpeed => None,
            AppMenuItem::SlowSpeed => None,
            AppMenuItem::RewindSize => None,
            AppMenuItem::RewindInterval => None,
            AppMenuItem::AudioBufferSize => None,
            AppMenuItem::MuteTurbo => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteTurbo)),
            AppMenuItem::MuteSlow => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::MuteSlow)),
            AppMenuItem::DefaultConfig => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Default)),
            AppMenuItem::RestartGame => Some(AppCmd::RestartGame),
            AppMenuItem::InputMenu => {
                self.next_items(input_menu());

                None
            }
            AppMenuItem::ComboInterval => None,
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PaletteInverted))
            }
            AppMenuItem::FrameBlendAlpha => None,
            AppMenuItem::FrameBlendMode => None,
            AppMenuItem::FrameBlendFade => None,
            AppMenuItem::FrameBlendDim => None,
            AppMenuItem::VideoMenu => {
                self.next_items(video_menu(&config.video.frame_blend_mode));

                None
            }
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
            AppMenuItem::Quit => "Quit".to_string(),
            AppMenuItem::SaveState => format!("Save({})", config.current_save_index),
            AppMenuItem::LoadState => format!("Load({})", config.current_load_index),
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
                format!("Scale(x{})", config.interface.scale)
            }
            AppMenuItem::DeveloperMenu => "Developer".to_string(),
            AppMenuItem::TileWindow => {
                format!("Tile Window{}", get_suffix(config.interface.tile_window))
            }
            AppMenuItem::SpinDuration => {
                format!(
                    "Spin Interval({}ns)",
                    config.get_emu_config().spin_duration.as_nanos()
                )
            }
            AppMenuItem::SystemMenu => "System".to_string(),
            AppMenuItem::AutoSaveState => {
                format!("Auto Save State{}", get_suffix(config.auto_save_state))
            }
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
                "Rewind Interval({}s)",
                config.emulation.rewind_interval.as_secs()
            ),
            AppMenuItem::AudioBufferSize => format!("Buffer Size({})", config.audio.buffer_size),
            AppMenuItem::MuteTurbo => format!("Mute Turbo{}", get_suffix(config.audio.mute_turbo)),
            AppMenuItem::MuteSlow => format!("Mute Slow{}", get_suffix(config.audio.mute_slow)),
            AppMenuItem::DefaultConfig => "Reset Default".to_string(),
            AppMenuItem::RestartGame => "Restart".to_string(),
            AppMenuItem::InputMenu => "Input".to_string(),
            AppMenuItem::ComboInterval => format!(
                "Combo Interval({}ms)",
                config.input.combo_interval.as_millis()
            ),
            AppMenuItem::PaletteInverted => format!(
                "Palette Inverted{}",
                get_suffix(config.interface.is_palette_inverted)
            ),
            AppMenuItem::FrameBlendMode => {
                format!("Frame Blend({})", config.video.frame_blend_mode.get_name())
            }
            AppMenuItem::FrameBlendAlpha => {
                format!("Frame Alpha({})", config.video.frame_blend_mode.get_alpha())
            }
            AppMenuItem::FrameBlendFade => {
                format!("Frame Fade({})", config.video.frame_blend_mode.get_fade())
            }
            AppMenuItem::FrameBlendDim => {
                format!("Frame Dim({})", config.video.frame_blend_mode.get_dim())
            }
            AppMenuItem::VideoMenu => "Video".to_string(),
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
