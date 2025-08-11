use std::path::Path;
use crate::app::AppCmd;
use crate::config::{VideoBackendType, VideoConfig};
use crate::menu::files::FilesMenu;
use crate::menu::item::AppMenuItem;
use crate::menu::roms::RomsMenu;
use crate::menu::SubMenu;
use crate::video::frame_blend::FrameBlendMode;
use crate::PlatformFileSystem;
use crate::roms::RomsState;

pub fn video_menu(conf: &VideoConfig) -> Box<[AppMenuItem]> {
    let mut items = Vec::with_capacity(15);
    items.push(AppMenuItem::VideoBackend);

    if conf.render.backend == VideoBackendType::Sdl2 {
        items.push(AppMenuItem::GridFilter);
        items.push(AppMenuItem::SubpixelFilter);
        items.push(AppMenuItem::ScanlineFilter);
        items.push(AppMenuItem::DotMatrixFilter);
        items.push(AppMenuItem::VignetteFilter);
    } else if conf.render.backend == VideoBackendType::Gl {
        items.push(AppMenuItem::VideoShader);
        items.push(AppMenuItem::ShaderFrameBlend);
    }

    items.push(AppMenuItem::CpuFrameBlendMode);

    match conf.render.frame_blend_mode {
        FrameBlendMode::None => {}
        FrameBlendMode::Linear(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendAlpha);
        }
        FrameBlendMode::Exp(_) => {
            items.push(AppMenuItem::FrameBlendDim);
            items.push(AppMenuItem::FrameBlendFade);
        }
        FrameBlendMode::Gamma(_) | FrameBlendMode::Additive(_) => {
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

pub fn roms_menu(filesystem: &impl PlatformFileSystem, roms: &RomsState) -> Box<[AppMenuItem]> {
    let roms: Box<dyn SubMenu> = Box::new(RomsMenu::new(filesystem, roms));

    vec![AppMenuItem::RomsSubMenu(roms), AppMenuItem::Back].into_boxed_slice()
}

pub fn files_menu(_filesystem: &impl PlatformFileSystem, last_path: Option<impl AsRef<Path>>) -> Box<[AppMenuItem]> {
    let files: Box<dyn SubMenu> = Box::new(FilesMenu::new(last_path));

    vec![AppMenuItem::FileBrowserSubMenu(files)].into_boxed_slice()
}

pub fn input_menu() -> Box<[AppMenuItem]> {
    vec![AppMenuItem::ComboInterval, AppMenuItem::Back].into_boxed_slice()
}

pub fn confirm_menu(cmd: AppCmd) -> Box<[AppMenuItem]> {
    vec![AppMenuItem::Confirm(cmd), AppMenuItem::Back].into_boxed_slice()
}

pub fn system_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::AutoSaveState,
        AppMenuItem::NormalSpeed,
        AppMenuItem::TurboSpeed,
        AppMenuItem::SlowSpeed,
        AppMenuItem::RewindSize,
        AppMenuItem::RewindInterval,
        #[cfg(feature = "file-dialog")]
        AppMenuItem::RomsDir,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

pub fn advanced_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::TileWindow,
        AppMenuItem::SpinDuration,
        AppMenuItem::ResetConfig,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

pub fn start_menu() -> Box<[AppMenuItem]> {
    vec![
        #[cfg(feature = "file-dialog")]
        AppMenuItem::OpenRom,
        AppMenuItem::RomsMenu,
        #[cfg(feature = "file-browser")]
        AppMenuItem::FileBrowser,
        AppMenuItem::SettingsMenu,
        AppMenuItem::Quit,
    ]
    .into_boxed_slice()
}

pub fn settings_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::InterfaceMenu,
        AppMenuItem::VideoMenu,
        AppMenuItem::AudioMenu,
        AppMenuItem::InputMenu,
        AppMenuItem::SystemMenu,
        AppMenuItem::AdvancedMenu,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}

pub fn interface_menu() -> Box<[AppMenuItem]> {
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

pub fn game_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Resume,
        AppMenuItem::SaveState,
        AppMenuItem::LoadState,
        AppMenuItem::RestartGame,
        #[cfg(feature = "file-dialog")]
        AppMenuItem::OpenRom,
        AppMenuItem::RomsMenu,
        #[cfg(feature = "file-browser")]
        AppMenuItem::FileBrowser,
        AppMenuItem::SettingsMenu,
        AppMenuItem::Quit,
    ]
    .into_boxed_slice()
}

pub fn audio_menu() -> Box<[AppMenuItem]> {
    vec![
        AppMenuItem::Volume,
        AppMenuItem::AudioBufferSize,
        AppMenuItem::MuteTurbo,
        AppMenuItem::MuteSlow,
        AppMenuItem::Back,
    ]
    .into_boxed_slice()
}
