use crate::app::AppCmd;
use crate::config::AppConfig;
use crate::menu::{get_menu_item_suffix, truncate_menu_item, SubMenu};

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
    AdvancedMenu,
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
    CpuFrameBlendMode,
    FrameBlendAlpha,
    FrameBlendFade,
    FrameBlendDim,
    VideoMenu,
    FrameBlendProfile,
    FrameBlendRise,
    FrameBlendFall,
    FrameBlendBleed,
    GridFilter,
    SubpixelFilter,
    RomsMenu,
    RomsSubMenu(Box<dyn SubMenu>),
    RomsDir,
    Confirm(AppCmd),
    ScanlineFilter,
    DotMatrixFilter,
    VignetteFilter,
    VideoBackend,
    VideoShader,
    ShaderFrameBlend,
    FileBrowser,
    FileBrowserSubMenu(Box<dyn SubMenu>),
}

impl AppMenuItem {
    pub fn get_items_mut(&mut self) -> Option<&mut Box<dyn SubMenu>> {
        match self {
            AppMenuItem::Resume
            | AppMenuItem::Confirm(_)
            | AppMenuItem::RomsDir
            | AppMenuItem::ShaderFrameBlend
            | AppMenuItem::VideoBackend
            | AppMenuItem::VideoShader
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
            | AppMenuItem::VignetteFilter
            | AppMenuItem::Volume
            | AppMenuItem::Scale
            | AppMenuItem::AdvancedMenu
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
            | AppMenuItem::CpuFrameBlendMode
            | AppMenuItem::FrameBlendAlpha
            | AppMenuItem::FrameBlendFade
            | AppMenuItem::FrameBlendDim
            | AppMenuItem::VideoMenu
            | AppMenuItem::FrameBlendProfile
            | AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed
            | AppMenuItem::GridFilter
            | AppMenuItem::SubpixelFilter
            | AppMenuItem::ScanlineFilter
            | AppMenuItem::DotMatrixFilter
            | AppMenuItem::FileBrowser
            | AppMenuItem::RomsMenu => None,
            AppMenuItem::RomsSubMenu(x) => Some(x),
            AppMenuItem::FileBrowserSubMenu(x) => Some(x),
        }
    }

    pub fn get_items(&self) -> Option<&Box<dyn SubMenu>> {
        match self {
            AppMenuItem::Resume
            | AppMenuItem::Confirm(_)
            | AppMenuItem::SaveState
            | AppMenuItem::ShaderFrameBlend
            | AppMenuItem::RomsDir
            | AppMenuItem::VideoBackend
            | AppMenuItem::VideoShader
            | AppMenuItem::LoadState
            | AppMenuItem::OpenRom
            | AppMenuItem::VignetteFilter
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
            | AppMenuItem::AdvancedMenu
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
            | AppMenuItem::CpuFrameBlendMode
            | AppMenuItem::FrameBlendAlpha
            | AppMenuItem::FrameBlendFade
            | AppMenuItem::FrameBlendDim
            | AppMenuItem::VideoMenu
            | AppMenuItem::FrameBlendProfile
            | AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed
            | AppMenuItem::GridFilter
            | AppMenuItem::SubpixelFilter
            | AppMenuItem::ScanlineFilter
            | AppMenuItem::DotMatrixFilter
            | AppMenuItem::FileBrowser
            | AppMenuItem::RomsMenu => None,
            AppMenuItem::RomsSubMenu(x) => Some(x),
            AppMenuItem::FileBrowserSubMenu(x) => Some(x),
        }
    }
}

impl AppMenuItem {
    pub fn to_string(&self, config: &AppConfig) -> String {
        let item_str = match self {
            AppMenuItem::Resume => "Resume".to_string(),
            AppMenuItem::OpenRom => "Open ROM".to_string(),
            AppMenuItem::Quit => "Quit".to_string(),
            AppMenuItem::SaveState => format!("Save({})", config.current_save_index),
            AppMenuItem::LoadState => format!("Load({})", config.current_load_index),
            AppMenuItem::SettingsMenu => "Settings".to_string(),
            AppMenuItem::InterfaceMenu => "Interface".to_string(),
            AppMenuItem::Back => "Back".to_string(),
            AppMenuItem::Palette => {
                format!("Palette({})", config.video.interface.selected_palette_idx)
            }
            AppMenuItem::ToggleFps => format!(
                "FPS{}",
                get_menu_item_suffix(config.video.interface.show_fps)
            ),
            AppMenuItem::ToggleFullscreen => {
                format!(
                    "Fullscreen{}",
                    get_menu_item_suffix(config.video.interface.is_fullscreen)
                )
            }
            AppMenuItem::AudioMenu => "Audio".to_string(),
            AppMenuItem::Volume => {
                format!("Volume({})", (config.audio.volume * 100.0) as i32)
            }
            AppMenuItem::Scale => {
                format!("Scale(x{})", config.video.interface.scale)
            }
            AppMenuItem::AdvancedMenu => "Advanced".to_string(),
            AppMenuItem::TileWindow => {
                format!(
                    "Show Tiles{}",
                    get_menu_item_suffix(config.video.interface.show_tiles)
                )
            }
            AppMenuItem::SpinDuration => {
                format!(
                    "Spin Wait({}µs)",
                    config.get_emu_config().spin_duration.as_micros()
                )
            }
            AppMenuItem::SystemMenu => "System".to_string(),
            AppMenuItem::AutoSaveState => {
                format!(
                    "Auto Save State{}",
                    get_menu_item_suffix(config.auto_save_state)
                )
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
            AppMenuItem::MuteTurbo => format!(
                "Mute Turbo{}",
                get_menu_item_suffix(config.audio.mute_turbo)
            ),
            AppMenuItem::MuteSlow => {
                format!("Mute Slow{}", get_menu_item_suffix(config.audio.mute_slow))
            }
            AppMenuItem::ResetConfig => "Reset Settings".to_string(),
            AppMenuItem::RestartGame => "Restart".to_string(),
            AppMenuItem::InputMenu => "Input".to_string(),
            AppMenuItem::ComboInterval => format!(
                "Combo Interval({}ms)",
                config.input.combo_interval.as_millis()
            ),
            AppMenuItem::PaletteInverted => format!(
                "Palette Inverted{}",
                get_menu_item_suffix(config.video.interface.is_palette_inverted)
            ),
            AppMenuItem::CpuFrameBlendMode => {
                format!(
                    "CPU Frame Blend({})",
                    config.video.render.frame_blend_mode.get_name()
                )
            }
            AppMenuItem::FrameBlendAlpha => {
                format!(
                    "Blend Alpha({})",
                    config.video.render.frame_blend_mode.get_alpha()
                )
            }
            AppMenuItem::FrameBlendFade => {
                format!(
                    "Blend Fade({})",
                    config.video.render.frame_blend_mode.get_fade()
                )
            }
            AppMenuItem::FrameBlendDim => {
                format!("Blend Dim({})", config.video.render.blend_dim)
            }
            AppMenuItem::VideoMenu => "Video".to_string(),
            AppMenuItem::FrameBlendProfile => {
                format!(
                    "Blend Profile({})",
                    config
                        .video
                        .render
                        .frame_blend_mode
                        .get_profile()
                        .unwrap()
                        .name()
                )
            }
            AppMenuItem::FrameBlendRise => format!(
                "Blend Rise({})",
                config
                    .video
                    .render
                    .frame_blend_mode
                    .get_profile()
                    .unwrap()
                    .rise
            ),
            AppMenuItem::FrameBlendFall => format!(
                "Blend Fall({})",
                config
                    .video
                    .render
                    .frame_blend_mode
                    .get_profile()
                    .unwrap()
                    .fall
            ),
            AppMenuItem::FrameBlendBleed => format!(
                "Blend Bleed({})",
                config
                    .video
                    .render
                    .frame_blend_mode
                    .get_profile()
                    .unwrap()
                    .bleed
            ),
            AppMenuItem::GridFilter => {
                format!(
                    "Grid{}",
                    get_menu_item_suffix(config.video.render.sdl2.grid_enabled)
                )
            }
            AppMenuItem::SubpixelFilter => {
                format!(
                    "Mask{}",
                    get_menu_item_suffix(config.video.render.sdl2.subpixel_enabled)
                )
            }
            AppMenuItem::RomsMenu => "ROMs".to_string(),
            AppMenuItem::RomsSubMenu(_) => "ROMs Sub".to_string(),
            AppMenuItem::RomsDir => "Select ROMs Dir".to_string(),
            AppMenuItem::Confirm(_) => "Confirm".to_string(),
            AppMenuItem::ScanlineFilter => {
                format!(
                    "Scanline{}",
                    get_menu_item_suffix(config.video.render.sdl2.scanline_enabled)
                )
            }
            AppMenuItem::DotMatrixFilter => {
                format!(
                    "Dot-Matrix{}",
                    get_menu_item_suffix(config.video.render.sdl2.dot_matrix_enabled)
                )
            }
            AppMenuItem::VignetteFilter => {
                format!(
                    "Vignette{}",
                    get_menu_item_suffix(config.video.render.sdl2.vignette_enabled)
                )
            }
            AppMenuItem::VideoBackend => {
                format!("Backend({:?})", config.video.render.backend)
            }
            AppMenuItem::VideoShader => {
                format!("Shader({:?})", config.video.render.gl.shader_name)
            }
            AppMenuItem::ShaderFrameBlend => {
                format!(
                    "GPU Frame Blend({:?})",
                    config.video.render.gl.shader_frame_blend_mode
                )
            }
            AppMenuItem::FileBrowser => "Browse".to_string(),
            AppMenuItem::FileBrowserSubMenu(_) => "Browse Sub".to_string(),
        };

        truncate_menu_item(&item_str)
    }
}
