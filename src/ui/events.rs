use crate::bus::Bus;
use crate::config::GraphicsConfig;

pub trait UiEventHandler {
    fn on_event(&mut self, bus: &mut Bus, event: UiEvent);
}

pub enum UiEvent {
    Quit,
    Pause,
    DropFile(String),
    Restart,
    ConfigChanged(GraphicsConfig),
}
