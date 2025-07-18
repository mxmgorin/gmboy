use core::emu::ctx::EmuState;
use core::emu::ctx::EmuCtx;
use core::emu::save_state::SaveStateEvent;
use core::emu::ctx::RunMode;
use std::path::PathBuf;
use core::bus::Bus;
use core::emu::config::GraphicsConfig;

pub trait UiEventHandler {
    fn on_event(&mut self, bus: &mut Bus, event: UiEvent);
}

pub enum UiEvent {
    Quit,
    Pause,
    FileDropped(PathBuf),
    Restart,
    ConfigChanged(GraphicsConfig),
    ModeChanged(RunMode),
    Mute,
    SaveState(SaveStateEvent, usize),
    PickFile,
}

impl UiEventHandler for EmuCtx {
    fn on_event(&mut self, _bus: &mut Bus, event: UiEvent) {
        match event {
            UiEvent::Quit => self.state = EmuState::Quit,
            UiEvent::FileDropped(path) => self.state = EmuState::LoadCart(path),
            UiEvent::Pause => {
                if self.state == EmuState::Paused {
                    self.state = EmuState::Running(RunMode::Normal);
                } else {
                    self.state = EmuState::Paused;
                }
            }
            UiEvent::Restart => {
                if let Some(path) = &self.config.last_cart_path {
                    self.state = EmuState::LoadCart(PathBuf::from(path));
                }
            }
            UiEvent::ConfigChanged(config) => self.config.graphics = config,
            UiEvent::ModeChanged(mode) => self.state = EmuState::Running(mode),
            UiEvent::Mute => self.config.emulation.is_muted = !self.config.emulation.is_muted,
            UiEvent::SaveState(event, index) => self.pending_save_state = Some((event, index)),
            UiEvent::PickFile => {
                if self.state == EmuState::WaitCart || self.state == EmuState::Paused {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.state = EmuState::LoadCart(path);
                    }
                }
            }
        }
    }
}