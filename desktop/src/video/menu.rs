use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::config::AppConfig;
use crate::roms::RomsList;
use crate::video::frame_blend::{
    AdditiveFrameBlend, ExponentialFrameBlend, FrameBlendMode, GammaCorrectedFrameBlend,
    LinearFrameBlend,
};
use crate::video::frame_blend::{DMG_PROFILE, POCKET_PROFILE};
use std::collections::VecDeque;
use std::mem;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AppMenuItem {
    Resume,
    SaveState,
    LoadState,
    OpenRom,
    SettingsMenu,
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
    ResetConfig,
    RestartGame,
    InputMenu,
    ComboInterval,
    PaletteInverted,
    FrameBlendMode,
    FrameBlendAlpha,
    FrameBlendFade,
    FrameBlendDim,
    VideoMenu,
    FrameBlendProfile,
    FrameBlendRise,
    FrameBlendFall,
    FrameBlendBleed,
    PixelGrid,
    PixelMask,
    RomsMenu,
    Roms(RomsMenu),
    RomsDir,
    Confirm(AppCmd),
    Tools,
}

impl AppMenuItem {
    pub fn get_inner_mut(&mut self) -> Option<&mut RomsMenu> {
        match self {
            AppMenuItem::Resume
            | AppMenuItem::Tools
            | AppMenuItem::Confirm(_)
            | AppMenuItem::RomsDir
            | AppMenuItem::SaveState
            | AppMenuItem::LoadState
            | AppMenuItem::OpenRom
            | AppMenuItem::SettingsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Quit
            | AppMenuItem::Palette
            | AppMenuItem::ToggleFps
            | AppMenuItem::ToggleFullscreen
            | AppMenuItem::AudioMenu
            | AppMenuItem::Volume
            | AppMenuItem::Scale
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::TileWindow
            | AppMenuItem::SpinDuration
            | AppMenuItem::SystemMenu
            | AppMenuItem::AutoSaveState
            | AppMenuItem::NormalSpeed
            | AppMenuItem::TurboSpeed
            | AppMenuItem::SlowSpeed
            | AppMenuItem::RewindSize
            | AppMenuItem::RewindInterval
            | AppMenuItem::AudioBufferSize
            | AppMenuItem::MuteTurbo
            | AppMenuItem::MuteSlow
            | AppMenuItem::ResetConfig
            | AppMenuItem::RestartGame
            | AppMenuItem::InputMenu
            | AppMenuItem::ComboInterval
            | AppMenuItem::PaletteInverted
            | AppMenuItem::FrameBlendMode
            | AppMenuItem::FrameBlendAlpha
            | AppMenuItem::FrameBlendFade
            | AppMenuItem::FrameBlendDim
            | AppMenuItem::VideoMenu
            | AppMenuItem::FrameBlendProfile
            | AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed
            | AppMenuItem::PixelGrid
            | AppMenuItem::PixelMask
            | AppMenuItem::RomsMenu => None,
            AppMenuItem::Roms(x) => Some(x),
        }
    }

    pub fn get_inner(&self) -> Option<&RomsMenu> {
        match self {
            AppMenuItem::Resume
            | AppMenuItem::Tools
            | AppMenuItem::Confirm(_)
            | AppMenuItem::SaveState
            | AppMenuItem::RomsDir
            | AppMenuItem::LoadState
            | AppMenuItem::OpenRom
            | AppMenuItem::SettingsMenu
            | AppMenuItem::InterfaceMenu
            | AppMenuItem::Back
            | AppMenuItem::Quit
            | AppMenuItem::Palette
            | AppMenuItem::ToggleFps
            | AppMenuItem::ToggleFullscreen
            | AppMenuItem::AudioMenu
            | AppMenuItem::Volume
            | AppMenuItem::Scale
            | AppMenuItem::DeveloperMenu
            | AppMenuItem::TileWindow
            | AppMenuItem::SpinDuration
            | AppMenuItem::SystemMenu
            | AppMenuItem::AutoSaveState
            | AppMenuItem::NormalSpeed
            | AppMenuItem::TurboSpeed
            | AppMenuItem::SlowSpeed
            | AppMenuItem::RewindSize
            | AppMenuItem::RewindInterval
            | AppMenuItem::AudioBufferSize
            | AppMenuItem::MuteTurbo
            | AppMenuItem::MuteSlow
            | AppMenuItem::ResetConfig
            | AppMenuItem::RestartGame
            | AppMenuItem::InputMenu
            | AppMenuItem::ComboInterval
            | AppMenuItem::PaletteInverted
            | AppMenuItem::FrameBlendMode
            | AppMenuItem::FrameBlendAlpha
            | AppMenuItem::FrameBlendFade
            | AppMenuItem::FrameBlendDim
            | AppMenuItem::VideoMenu
            | AppMenuItem::FrameBlendProfile
            | AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed
            | AppMenuItem::PixelGrid
            | AppMenuItem::PixelMask
            | AppMenuItem::RomsMenu => None,
            AppMenuItem::Roms(x) => Some(x),
        }
    }
}

fn video_menu(frame_blend_type: &FrameBlendMode) -> Box<[AppMenuItem]> {
    let mut items = Vec::with_capacity(9);
    items.push(AppMenuItem::PixelGrid);
    items.push(AppMenuItem::PixelMask);
    items.push(AppMenuItem::FrameBlendMode);

    match frame_blend_type {
        FrameBlendMode::None => {}
        FrameBlendMode::Linear(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendAlpha);
        }
        FrameBlendMode::Exponential(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendFade);
        }
        FrameBlendMode::GammaCorrected(_) | FrameBlendMode::Additive(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendFade);
            items.push(AppMenuItem::FrameBlendAlpha);
        }
        FrameBlendMode::Accurate(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendProfile);
            items.push(AppMenuItem::FrameBlendFall);
            items.push(AppMenuItem::FrameBlendRise);
            items.push(AppMenuItem::FrameBlendBleed);
        }
    }

    items.push(AppMenuItem::Back);

    items.into_boxed_slice()
}

fn library_menu() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::Roms(RomsMenu::default()), AppMenuItem::Back].into_boxed_slice()
}

fn input_menu() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::ComboInterval, AppMenuItem::Back].into_boxed_slice()
}

fn confirm_menu(cmd: AppCmd) -> Box<[AppMenuItem]> {
    vec![AppMenuItem::Confirm(cmd), AppMenuItem::Back].into_boxed_slice()
}

fn tools_menu() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::ResetConfig, AppMenuItem::Back].into_boxed_slice()
}

fn system_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::AutoSaveState,
        AppMenuItem::NormalSpeed,
        AppMenuItem::TurboSpeed,
        AppMenuItem::SlowSpeed,
        AppMenuItem::RewindSize,
        AppMenuItem::RewindInterval,
        AppMenuItem::RomsDir,
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
        AppMenuItem::RomsMenu,
        AppMenuItem::SettingsMenu,
        AppMenuItem::Quit,
    ]
    .into_boxed_slice()
}

fn settings_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::InterfaceMenu,
        AppMenuItem::VideoMenu,
        AppMenuItem::AudioMenu,
        AppMenuItem::InputMenu,
        AppMenuItem::SystemMenu,
        AppMenuItem::Tools,
        AppMenuItem::DeveloperMenu,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

fn interface_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Palette,
        AppMenuItem::PaletteInverted,
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
        AppMenuItem::SaveState,
        AppMenuItem::LoadState,
        AppMenuItem::RestartGame,
        AppMenuItem::OpenRom,
        AppMenuItem::RomsMenu,
        AppMenuItem::SettingsMenu,
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

#[derive(Debug, Clone, Default)]
struct MenuBuffer {
    items: Vec<String>,
    refs: Vec<*const str>,
}

impl MenuBuffer {
    pub fn add(&mut self, item: impl Into<String>) {
        let item = item.into();
        let ptr: *const str = item.as_str();
        self.refs.push(ptr);
        self.items.push(item);
    }

    pub fn get(&self) -> &[&str] {
        // SAFETY:
        // all pointers in `self.refs` point to valid strings in `self.items`
        // Convert &[ *const str ] -> &[ &str ] by transmuting each element
        unsafe { std::slice::from_raw_parts(self.refs.as_ptr() as *const &str, self.refs.len()) }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.refs.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub struct AppMenu {
    prev_items: VecDeque<Box<[AppMenuItem]>>,
    items: Box<[AppMenuItem]>,
    selected_index: usize,
    buffer: MenuBuffer,
    updated: bool,
    inner_buffer: MenuBuffer,
}

impl AppMenu {
    pub fn new(with_cart: bool) -> Self {
        Self {
            prev_items: VecDeque::with_capacity(4),
            items: if with_cart { game_menu() } else { start_menu() },
            selected_index: 0,
            buffer: MenuBuffer::default(),
            updated: true,
            inner_buffer: Default::default(),
        }
    }

    pub fn request_update(&mut self) {
        self.updated = true;
    }

    pub fn get_items(&mut self, config: &AppConfig) -> (&[&str], bool) {
        let updated = self.updated;
        self.updated = false;

        if updated {
            self.buffer.clear();
            self.inner_buffer.clear();

            for (i, item) in self.items.iter_mut().enumerate() {
                if let Some(inner) = item.get_inner() {
                    for inner_item in inner.get_iterator() {
                        self.inner_buffer.add(inner_item);
                    }

                    return (self.inner_buffer.get(), updated);
                } else {
                    let line = item.to_string(config);
                    if i == self.selected_index {
                        self.buffer.add(format!("◀{line}▶"));
                    } else {
                        self.buffer.add(line.to_string());
                    }
                }
            }
        } else if !self.inner_buffer.is_empty() {
            return (self.inner_buffer.get(), updated);
        }

        (self.buffer.get(), updated)
    }

    pub fn move_up(&mut self) {
        self.updated = true;

        if let Some(curr) = self.items.get_mut(self.selected_index) {
            if let Some(inner) = curr.get_inner_mut() {
                inner.move_up();
                return;
            }
        }

        self.selected_index = core::move_prev_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn move_down(&mut self) {
        self.updated = true;
        if let Some(curr) = self.items.get_mut(self.selected_index) {
            if let Some(inner) = curr.get_inner_mut() {
                inner.move_down();
                return;
            }
        }

        self.selected_index = core::move_next_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn move_right(&mut self, config: &AppConfig) -> Option<AppCmd> {
        self.updated = true;
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::SaveState => {
                let i = core::move_next_wrapped(config.current_save_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SetSaveIndex(i)))
            }
            AppMenuItem::LoadState => {
                let i = core::move_next_wrapped(config.current_load_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SetLoadIndex(i)))
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
            | AppMenuItem::SettingsMenu
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
            AppMenuItem::ResetConfig => None,
            AppMenuItem::RestartGame => None,
            AppMenuItem::InputMenu => None,
            AppMenuItem::ComboInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::ComboInterval(5_000),
            )),
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette))
            }
            AppMenuItem::FrameBlendMode => {
                let mode = match config.video.frame_blend_mode {
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
                    FrameBlendMode::GammaCorrected(_) => FrameBlendMode::Accurate(DMG_PROFILE),
                    FrameBlendMode::Accurate(_) => FrameBlendMode::None,
                };
                self.items = video_menu(&mode);
                let mut conf = config.video.clone();
                conf.frame_blend_mode = mode;

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendAlpha => {
                let mut conf = config.video.clone();
                conf.frame_blend_mode.change_alpha(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFade => {
                let mut conf = config.video.clone();
                conf.frame_blend_mode.change_fade(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendDim => {
                let mut conf = config.video.clone();
                conf.change_dim(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoMenu => None,
            AppMenuItem::FrameBlendProfile => {
                let mut conf = config.video.clone();

                if let FrameBlendMode::Accurate(x) = &mut conf.frame_blend_mode {
                    if x == &DMG_PROFILE {
                        *x = POCKET_PROFILE;
                    } else {
                        *x = DMG_PROFILE;
                    }
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendRise => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.rise = core::change_f32_rounded(profile.rise, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFall => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.fall = core::change_f32_rounded(profile.fall, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendBleed => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.bleed = core::change_f32_rounded(profile.bleed, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::PixelGrid => {
                let mut conf = config.video.clone();
                conf.grid_enabled = !conf.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::PixelMask => {
                let mut conf = config.video.clone();
                conf.mask_enabled = !conf.mask_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::RomsMenu => None,
            AppMenuItem::Roms(x) => x.move_right(),
            AppMenuItem::RomsDir => None,
            AppMenuItem::Confirm(_) => None,
            AppMenuItem::Tools => None,
        }
    }

    pub fn move_left(&mut self, config: &AppConfig) -> Option<AppCmd> {
        self.updated = true;
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::Tools => None,
            AppMenuItem::SaveState => {
                let i = core::move_prev_wrapped(config.current_save_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SetSaveIndex(i)))
            }
            AppMenuItem::LoadState => {
                let i = core::move_prev_wrapped(config.current_load_index, 99);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SetLoadIndex(i)))
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
            | AppMenuItem::SettingsMenu
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
            AppMenuItem::ResetConfig => None,
            AppMenuItem::RestartGame => None,
            AppMenuItem::InputMenu => None,
            AppMenuItem::ComboInterval => Some(AppCmd::ChangeConfig(
                ChangeAppConfigCmd::ComboInterval(-5_000),
            )),
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette))
            }
            AppMenuItem::FrameBlendAlpha => {
                let mut conf = config.video.clone();
                conf.frame_blend_mode.change_alpha(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendMode => {
                let blend_mode = match config.video.frame_blend_mode {
                    FrameBlendMode::None => FrameBlendMode::Accurate(DMG_PROFILE),
                    FrameBlendMode::Linear(_) => FrameBlendMode::None,
                    FrameBlendMode::Additive(_) => {
                        FrameBlendMode::Linear(LinearFrameBlend::default())
                    }
                    FrameBlendMode::Exponential(_) => {
                        FrameBlendMode::Additive(AdditiveFrameBlend::default())
                    }
                    FrameBlendMode::GammaCorrected(_) => {
                        FrameBlendMode::Exponential(ExponentialFrameBlend::default())
                    }
                    FrameBlendMode::Accurate(_) => {
                        FrameBlendMode::GammaCorrected(GammaCorrectedFrameBlend::default())
                    }
                };

                self.items = video_menu(&blend_mode);
                let mut conf = config.video.clone();
                conf.frame_blend_mode = blend_mode;

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFade => {
                let mut conf = config.video.clone();
                conf.frame_blend_mode.change_fade(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendDim => {
                let mut conf = config.video.clone();
                conf.change_dim(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoMenu => None,
            AppMenuItem::FrameBlendProfile => {
                let mut conf = config.video.clone();

                if let FrameBlendMode::Accurate(x) = &mut conf.frame_blend_mode {
                    if x == &DMG_PROFILE {
                        *x = POCKET_PROFILE;
                    } else {
                        *x = DMG_PROFILE;
                    }
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendRise => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.rise = core::change_f32_rounded(profile.rise, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFall => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.fall = core::change_f32_rounded(profile.fall, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendBleed => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.bleed = core::change_f32_rounded(profile.bleed, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::PixelGrid => {
                let mut conf = config.video.clone();
                conf.grid_enabled = !conf.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::PixelMask => {
                let mut conf = config.video.clone();
                conf.mask_enabled = !conf.mask_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::RomsMenu => None,
            AppMenuItem::Roms(x) => x.move_left(),
            AppMenuItem::RomsDir => None,
            AppMenuItem::Confirm(_) => None,
        }
    }

    pub fn back(&mut self) {
        self.updated = true;
        if let Some(prev) = self.prev_items.pop_back() {
            self.selected_index = 0;
            self.items = prev;
        }
    }

    pub fn select(&mut self, config: &AppConfig) -> Option<AppCmd> {
        self.updated = true;
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::Resume => Some(AppCmd::ToggleMenu),
            AppMenuItem::OpenRom => Some(AppCmd::SelectRom),
            AppMenuItem::Quit => Some(AppCmd::Quit),
            AppMenuItem::SaveState => Some(AppCmd::SaveState(
                core::emu::state::SaveStateCmd::Create,
                None,
            )),
            AppMenuItem::LoadState => Some(AppCmd::SaveState(
                core::emu::state::SaveStateCmd::Load,
                None,
            )),
            AppMenuItem::SettingsMenu => {
                self.next_items(settings_menu());

                None
            }
            AppMenuItem::InterfaceMenu => {
                self.next_items(interface_menu());

                None
            }
            AppMenuItem::Back => {
                self.back();

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
            AppMenuItem::ResetConfig => {
                self.next_items(confirm_menu(AppCmd::ChangeConfig(
                    ChangeAppConfigCmd::Default,
                )));

                None
            }
            AppMenuItem::RestartGame => Some(AppCmd::RestartGame),
            AppMenuItem::InputMenu => {
                self.next_items(input_menu());

                None
            }
            AppMenuItem::ComboInterval => None,
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette))
            }
            AppMenuItem::FrameBlendAlpha => None,
            AppMenuItem::FrameBlendMode => None,
            AppMenuItem::FrameBlendFade => None,
            AppMenuItem::FrameBlendDim => None,
            AppMenuItem::VideoMenu => {
                self.next_items(video_menu(&config.video.frame_blend_mode));

                None
            }
            AppMenuItem::FrameBlendProfile => None,
            AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed => None,
            AppMenuItem::PixelGrid => {
                let mut conf = config.video.clone();
                conf.grid_enabled = !conf.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::PixelMask => {
                let mut conf = config.video.clone();
                conf.mask_enabled = !conf.mask_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::RomsMenu => {
                self.next_items(library_menu());

                None
            }
            AppMenuItem::Roms(x) => {
                let (cmd, is_back) = x.select(config);

                if is_back {
                    self.inner_buffer.clear();
                    self.back();
                }

                cmd
            }
            AppMenuItem::RomsDir => Some(AppCmd::SelectRomsDir),
            AppMenuItem::Confirm(cmd) => {
                let cmd = cmd.to_owned();
                self.back();

                Some(cmd)
            },
            AppMenuItem::Tools => {
                self.next_items(tools_menu());

                None
            },

        }
    }

    fn next_items(&mut self, items: Box<[AppMenuItem]>) {
        self.updated = true;
        let prev = mem::replace(&mut self.items, items);
        self.selected_index = 0;
        self.prev_items.push_front(prev);
    }
}

impl AppMenuItem {
    pub fn to_string(&self, config: &AppConfig) -> String {
        match self {
            AppMenuItem::Resume => "Resume".to_string(),
            AppMenuItem::OpenRom => "Open ROM".to_string(),
            AppMenuItem::Quit => "Quit".to_string(),
            AppMenuItem::SaveState => format!("Save({})", config.current_save_index),
            AppMenuItem::LoadState => format!("Load({})", config.current_load_index),
            AppMenuItem::SettingsMenu => "Settings".to_string(),
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
            AppMenuItem::ResetConfig => "Reset Settings".to_string(),
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
                format!("Frame Dim({})", config.video.dim)
            }
            AppMenuItem::VideoMenu => "Video".to_string(),
            AppMenuItem::FrameBlendProfile => {
                format!(
                    "Blend Profile({})",
                    config.video.frame_blend_mode.get_profile().unwrap().name()
                )
            }
            AppMenuItem::FrameBlendRise => format!(
                "Frame Rise({})",
                config.video.frame_blend_mode.get_profile().unwrap().rise
            ),
            AppMenuItem::FrameBlendFall => format!(
                "Frame Fall({})",
                config.video.frame_blend_mode.get_profile().unwrap().fall
            ),
            AppMenuItem::FrameBlendBleed => format!(
                "Frame Bleed({})",
                config.video.frame_blend_mode.get_profile().unwrap().bleed
            ),
            AppMenuItem::PixelGrid => format!("Grid{}", get_suffix(config.video.grid_enabled)),
            AppMenuItem::PixelMask => format!("Mask{}", get_suffix(config.video.mask_enabled)),
            AppMenuItem::RomsMenu => "ROMs".to_string(),
            AppMenuItem::Roms(x) => format!("ROMs ({})", x.items.len()),
            AppMenuItem::RomsDir => "Select ROMs Dir".to_string(),
            AppMenuItem::Confirm(_) => "Confirm".to_string(),
            AppMenuItem::Tools => "Tools".to_string(),
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

const MAX_ROMS_PER_PAGE: usize = 10;
const MAX_ROM_CHARS: usize = 16;

#[derive(Debug, Clone)]
pub struct RomsMenu {
    all_items: Box<[RomMenuItem]>, // all ROMs
    items: Box<[RomMenuItem]>,     // current page items (plus nav items)
    selected_index: usize,
    current_page: usize,
}

#[derive(Debug, Clone)]
pub struct RomMenuItem {
    pub name: String,
    pub path: PathBuf,
}

impl RomMenuItem {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| {
                let mut truncated: String = s.chars().take(MAX_ROM_CHARS).collect();
                if s.chars().count() > MAX_ROM_CHARS {
                    truncated.push_str("..");
                }
                truncated
            })
            .unwrap();

        Self { name, path }
    }
}

impl RomsMenu {
    pub fn get_iterator(&self) -> impl Iterator<Item = String> + '_ {
        self.items.iter().enumerate().map(move |(i, line)| {
            if i == self.selected_index {
                format!("◀{}▶", line.name)
            } else {
                line.name.clone()
            }
        })
    }

    pub fn move_up(&mut self) {
        self.selected_index = core::move_prev_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn move_down(&mut self) {
        self.selected_index = core::move_next_wrapped(self.selected_index, self.items.len() - 1);
    }

    pub fn move_left(&mut self) -> Option<AppCmd> {
        self.prev_page();

        None
    }

    pub fn move_right(&mut self) -> Option<AppCmd> {
        self.next_page();

        None
    }

    pub fn select(&mut self, _config: &AppConfig) -> (Option<AppCmd>, bool) {
        let item = &self.items[self.selected_index];

        if item.name.starts_with("Back") {
            return (None, true);
        }

        (Some(AppCmd::LoadFile(item.path.clone())), false)
    }

    pub fn next_page(&mut self) {
        let total_pages = self.all_items.len().div_ceil(MAX_ROMS_PER_PAGE);
        if self.current_page + 1 < total_pages {
            self.current_page += 1;
            self.update_page();
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.update_page();
        }
    }

    fn update_page(&mut self) {
        let prev_len = self.items.len();
        let total_pages = self.all_items.len().div_ceil(MAX_ROMS_PER_PAGE).max(1);
        let start = self.current_page * MAX_ROMS_PER_PAGE;
        let end = usize::min(start + MAX_ROMS_PER_PAGE, self.all_items.len());
        let mut page_items: Vec<RomMenuItem> = self.all_items[start..end].to_vec();

        page_items.push(RomMenuItem {
            name: format!("Page ({}/{})", self.current_page + 1, total_pages),
            path: Default::default(),
        });

        page_items.push(RomMenuItem {
            name: "Back".to_string(),
            path: Default::default(),
        });

        self.items = page_items.into_boxed_slice();

        if prev_len != self.items.len() {
            self.selected_index = self.items.len() - 2;
        }
    }
}

impl Default for RomsMenu {
    fn default() -> Self {
        let roms = RomsList::get_or_create();
        let mut all_items = Vec::with_capacity(12);

        for path in roms.get() {
            all_items.push(RomMenuItem::new(path));
        }

        all_items.sort_by(|a, b| a.name.cmp(&b.name));

        let mut menu = Self {
            items: Box::new([]),
            all_items: all_items.into_boxed_slice(),
            selected_index: 0,
            current_page: 0,
        };
        menu.update_page();

        menu
    }
}

#[cfg(test)]
mod tests {
    use crate::video::menu::{RomMenuItem, RomsMenu};

    #[test]
    pub fn iter() {
        let roms = RomsMenu {
            all_items: Box::new([]),
            items: vec![
                RomMenuItem::new("1"),
                RomMenuItem::new("2"),
                RomMenuItem::new("3"),
            ]
            .into_boxed_slice(),
            selected_index: 0,
            current_page: 0,
        };
        let mut iter = roms.get_iterator();

        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
