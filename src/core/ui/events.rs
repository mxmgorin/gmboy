use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};

pub trait UiEventHandler {
    fn on_event(&mut self, event: UiEvent);
}

pub enum UiEvent {
    Quit,
}

pub struct SdlEventHandler {
    pub event_pump: EventPump,
}

impl SdlEventHandler {
    pub fn new(sdl: &Sdl) -> Self {
        Self {
            event_pump: sdl.event_pump().unwrap(),
        }
    }

    pub fn handle(&mut self, event_handler: &mut impl UiEventHandler) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    ..
                } => event_handler.on_event(UiEvent::Quit),
                _ => {}
            }
        }
    }
}
