use crate::bus::Bus;
use sdl2::keyboard::Keycode;

pub trait UiEventHandler {
    fn on_event(&mut self, bus: &mut Bus, event: UiEvent);
}

pub enum UiEvent {
    Quit,
    KeyDown(Keycode),
}
