use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::config::{update_frame_skip, AppConfig, VideoBackendType};
use crate::menu::factory::{
    advanced_menu, audio_menu, confirm_menu, files_menu, input_menu, interface_menu,
    loaded_roms_menu, opened_roms_menu, settings_menu, system_menu, video_menu,
};
use crate::menu::item::AppMenuItem;
use crate::roms::RomsState;
use crate::video::frame_blend::{
    AdditiveFrameBlend, ExponentialFrameBlend, FrameBlendMode, GammaCorrectedFrameBlend,
    LinearFrameBlend, DMG_PROFILE, POCKET_PROFILE,
};
use crate::video::shader::ShaderFrameBlendMode;
use crate::PlatformFileSystem;

impl super::AppMenu {
    #[inline]
    pub fn move_up(&mut self) {
        self.updated = true;

        if let Some(curr) = self.items.get_mut(self.selected_index) {
            if let Some(inner) = curr.get_items_mut() {
                inner.move_up();
                return;
            }
        }

        self.selected_index = core::move_prev_wrapped(self.selected_index, self.items.len() - 1);
    }

    #[inline]
    pub fn move_down(&mut self) {
        self.updated = true;
        if let Some(curr) = self.items.get_mut(self.selected_index) {
            if let Some(inner) = curr.get_items_mut() {
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
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SpinDuration(100)))
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
            | AppMenuItem::AdvancedMenu
            | AppMenuItem::KeyboardInput
            | AppMenuItem::UpInput
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
            AppMenuItem::CpuFrameBlendMode => {
                let mut conf = config.video.clone();
                conf.render.frame_blend_mode = match config.video.render.frame_blend_mode {
                    FrameBlendMode::None => FrameBlendMode::Linear(LinearFrameBlend::default()),
                    FrameBlendMode::Linear(_) => {
                        FrameBlendMode::Additive(AdditiveFrameBlend::default())
                    }
                    FrameBlendMode::Additive(_) => {
                        FrameBlendMode::Exp(ExponentialFrameBlend::default())
                    }
                    FrameBlendMode::Exp(_) => {
                        FrameBlendMode::Gamma(GammaCorrectedFrameBlend::default())
                    }
                    FrameBlendMode::Gamma(_) => FrameBlendMode::Accurate(DMG_PROFILE),
                    FrameBlendMode::Accurate(_) => FrameBlendMode::None,
                };
                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendAlpha => {
                let mut conf = config.video.clone();
                conf.render.frame_blend_mode.change_alpha(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFade => {
                let mut conf = config.video.clone();
                conf.render.frame_blend_mode.change_fade(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendDim => {
                let mut conf = config.video.clone();
                conf.render.change_dim(0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoMenu => None,
            AppMenuItem::FrameBlendProfile => {
                let mut conf = config.video.clone();

                if let FrameBlendMode::Accurate(x) = &mut conf.render.frame_blend_mode {
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

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.rise = core::change_f32_rounded(profile.rise, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFall => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.fall = core::change_f32_rounded(profile.fall, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendBleed => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.bleed = core::change_f32_rounded(profile.bleed, 0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::GridFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.grid_enabled = !conf.render.sdl2.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::SubpixelFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.subpixel_enabled = !conf.render.sdl2.subpixel_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::BrowseRoms | AppMenuItem::LoadedRoms | AppMenuItem::OpenedRoms => None,
            AppMenuItem::BrowseRomsSubMenu(x)
            | AppMenuItem::LoadedRomsSubMenu(x)
            | AppMenuItem::OpenedRomsSubMenu(x) => x.move_right(),
            AppMenuItem::RomsDir => None,
            AppMenuItem::Confirm(_) => None,
            AppMenuItem::ScanlineFilter => None,
            AppMenuItem::DotMatrixFilter => None,
            AppMenuItem::VignetteFilter => None,
            AppMenuItem::VideoBackend => {
                let mut conf = config.video.clone();
                conf.render.backend = match config.video.render.backend {
                    VideoBackendType::Sdl2 => VideoBackendType::Gl,
                    VideoBackendType::Gl => VideoBackendType::Sdl2,
                };
                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoShader => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::NextShader)),
            AppMenuItem::ShaderFrameBlend => {
                let mut conf = config.video.clone();
                conf.render.gl.shader_frame_blend_mode =
                    match config.video.render.gl.shader_frame_blend_mode {
                        ShaderFrameBlendMode::None => ShaderFrameBlendMode::Simple,
                        ShaderFrameBlendMode::Simple => ShaderFrameBlendMode::AccEven,
                        ShaderFrameBlendMode::AccEven => ShaderFrameBlendMode::AccOdd,
                        ShaderFrameBlendMode::AccOdd => ShaderFrameBlendMode::None,
                    };
                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameSkip => {
                let frame_skip = update_frame_skip(config.video.render.frame_skip, 1);
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameSkip(
                    frame_skip,
                )))
            }
        }
    }

    pub fn move_left(&mut self, config: &AppConfig) -> Option<AppCmd> {
        self.updated = true;
        let item = self.items.get_mut(self.selected_index).unwrap();

        match item {
            AppMenuItem::ScanlineFilter => None,
            AppMenuItem::DotMatrixFilter => None,
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
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::SpinDuration(-100)))
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
            | AppMenuItem::AdvancedMenu
            | AppMenuItem::KeyboardInput
            | AppMenuItem::UpInput
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
                conf.render.frame_blend_mode.change_alpha(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::CpuFrameBlendMode => {
                let mut conf = config.video.clone();
                conf.render.frame_blend_mode = match config.video.render.frame_blend_mode {
                    FrameBlendMode::None => FrameBlendMode::Accurate(DMG_PROFILE),
                    FrameBlendMode::Linear(_) => FrameBlendMode::None,
                    FrameBlendMode::Additive(_) => {
                        FrameBlendMode::Linear(LinearFrameBlend::default())
                    }
                    FrameBlendMode::Exp(_) => {
                        FrameBlendMode::Additive(AdditiveFrameBlend::default())
                    }
                    FrameBlendMode::Gamma(_) => {
                        FrameBlendMode::Exp(ExponentialFrameBlend::default())
                    }
                    FrameBlendMode::Accurate(_) => {
                        FrameBlendMode::Gamma(GammaCorrectedFrameBlend::default())
                    }
                };

                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFade => {
                let mut conf = config.video.clone();
                conf.render.frame_blend_mode.change_fade(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendDim => {
                let mut conf = config.video.clone();
                conf.render.change_dim(-0.05);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoMenu => None,
            AppMenuItem::FrameBlendProfile => {
                let mut conf = config.video.clone();

                if let FrameBlendMode::Accurate(x) = &mut conf.render.frame_blend_mode {
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

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.rise = core::change_f32_rounded(profile.rise, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendFall => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.fall = core::change_f32_rounded(profile.fall, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameBlendBleed => {
                let mut conf = config.video.clone();

                if let Some(profile) = config.video.render.frame_blend_mode.get_profile() {
                    let mut profile = profile.clone();
                    profile.bleed = core::change_f32_rounded(profile.bleed, -0.05).clamp(0.0, 1.0);
                    profile.tint.reset();
                    conf.render.frame_blend_mode = FrameBlendMode::Accurate(profile);
                }

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::GridFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.grid_enabled = !conf.render.sdl2.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::SubpixelFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.subpixel_enabled = !conf.render.sdl2.subpixel_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::BrowseRoms | AppMenuItem::LoadedRoms | AppMenuItem::OpenedRoms => None,
            AppMenuItem::BrowseRomsSubMenu(x)
            | AppMenuItem::LoadedRomsSubMenu(x)
            | AppMenuItem::OpenedRomsSubMenu(x) => x.move_left(),
            AppMenuItem::RomsDir => None,
            AppMenuItem::Confirm(_) => None,
            AppMenuItem::VignetteFilter => None,
            AppMenuItem::VideoBackend => {
                let mut conf = config.video.clone();
                conf.render.backend = match config.video.render.backend {
                    VideoBackendType::Sdl2 => VideoBackendType::Gl,
                    VideoBackendType::Gl => VideoBackendType::Sdl2,
                };
                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoShader => Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevShader)),
            AppMenuItem::ShaderFrameBlend => {
                let mut conf = config.video.clone();
                conf.render.gl.shader_frame_blend_mode =
                    match config.video.render.gl.shader_frame_blend_mode {
                        ShaderFrameBlendMode::None => ShaderFrameBlendMode::AccOdd,
                        ShaderFrameBlendMode::Simple => ShaderFrameBlendMode::None,
                        ShaderFrameBlendMode::AccEven => ShaderFrameBlendMode::Simple,
                        ShaderFrameBlendMode::AccOdd => ShaderFrameBlendMode::AccEven,
                    };
                self.items = video_menu(&conf);

                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::FrameSkip => {
                let frame_skip = update_frame_skip(config.video.render.frame_skip, -1);
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::FrameSkip(
                    frame_skip,
                )))
            }
        }
    }

    pub fn back(&mut self) {
        self.updated = true;

        if !self.prev_items.is_empty() {
            let prev = self.prev_items.remove(self.prev_items.len() - 1);
            self.selected_index = 0;
            self.items = prev;
        }
    }

    pub fn select(
        &mut self,
        config: &AppConfig,
        filesystem: &impl PlatformFileSystem,
        roms: &RomsState,
    ) -> Option<AppCmd> {
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
            AppMenuItem::AdvancedMenu => {
                self.next_items(advanced_menu());

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
                    ChangeAppConfigCmd::Reset,
                )));

                None
            }
            AppMenuItem::RestartGame => Some(AppCmd::RestartRom),
            AppMenuItem::InputMenu => {
                self.next_items(input_menu());

                None
            }
            AppMenuItem::ComboInterval => None,
            AppMenuItem::PaletteInverted => {
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette))
            }
            AppMenuItem::FrameBlendAlpha => None,
            AppMenuItem::CpuFrameBlendMode => None,
            AppMenuItem::FrameBlendFade => None,
            AppMenuItem::FrameBlendDim => None,
            AppMenuItem::VideoMenu => {
                self.next_items(video_menu(&config.video));

                None
            }
            AppMenuItem::FrameBlendProfile => None,
            AppMenuItem::FrameBlendRise
            | AppMenuItem::FrameBlendFall
            | AppMenuItem::FrameBlendBleed => None,
            AppMenuItem::GridFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.grid_enabled = !conf.render.sdl2.grid_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::SubpixelFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.subpixel_enabled = !conf.render.sdl2.subpixel_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::BrowseRoms => {
                self.next_items(files_menu(filesystem, roms.last_browse_dir_path.as_ref()));

                None
            }
            AppMenuItem::LoadedRoms => {
                if roms.loaded_count() == 0 {
                    Some(AppCmd::SelectRomsDir)
                } else {
                    self.next_items(loaded_roms_menu(filesystem, roms));
                    None
                }
            }
            AppMenuItem::OpenedRoms => {
                self.next_items(opened_roms_menu(filesystem, roms));

                None
            }
            AppMenuItem::BrowseRomsSubMenu(x)
            | AppMenuItem::LoadedRomsSubMenu(x)
            | AppMenuItem::OpenedRomsSubMenu(x) => {
                let (cmd, is_back) = x.select(config);

                if is_back {
                    self.sub_buffer.clear();
                    self.back();
                }

                cmd
            }
            AppMenuItem::RomsDir => Some(AppCmd::SelectRomsDir),
            AppMenuItem::Confirm(cmd) => {
                let cmd = cmd.to_owned();
                self.back();

                Some(cmd)
            }
            AppMenuItem::ScanlineFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.scanline_enabled = !conf.render.sdl2.scanline_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::DotMatrixFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.dot_matrix_enabled = !conf.render.sdl2.dot_matrix_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VignetteFilter => {
                let mut conf = config.video.clone();
                conf.render.sdl2.vignette_enabled = !conf.render.sdl2.vignette_enabled;
                Some(AppCmd::ChangeConfig(ChangeAppConfigCmd::Video(conf)))
            }
            AppMenuItem::VideoBackend => None,
            AppMenuItem::VideoShader => None,
            AppMenuItem::ShaderFrameBlend => None,
            AppMenuItem::FrameSkip => None,
            AppMenuItem::KeyboardInput | AppMenuItem::UpInput => None,
        }
    }
}
