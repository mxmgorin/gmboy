use std::path::PathBuf;
use crate::bus::Bus;
use crate::config::GraphicsConfig;
use crate::emu::RunMode;

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

pub enum SaveStateEvent {
    Create,
    Load,
}
