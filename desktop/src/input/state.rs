use std::time::{Duration, Instant};
use sdl2::controller::Button;
use crate::app::{AppCmd};

pub struct ButtonState {
    pub pressed: bool,
    pub last_pressed: Instant,
    pub button: Button,
}

impl ButtonState {
    fn new(button: Button) -> Self {
        Self {
            button,
            pressed: false,
            last_pressed: Instant::now(),
        }
    }

    fn update(&mut self, is_pressed: bool) {
        if is_pressed && !self.pressed {
            self.last_pressed = Instant::now();
        }

        self.pressed = is_pressed;
    }
}

pub struct GamepadState {
    states: Vec<ButtonState>,
    combo_interval: Duration,
}

impl GamepadState {
    pub fn new(combo_interval: Duration) -> Self {
        Self {
            states: vec![
                ButtonState::new(Button::Start),
                ButtonState::new(Button::Back),
                ButtonState::new(Button::Guide),
            ],
            combo_interval,
        }
    }

    pub fn update(&mut self, button: Button, pressed: bool) {
        for state in self.states.iter_mut() {
            if state.button == button {
                state.update(pressed);
                break;
            }
        }
    }

    pub fn handle_combo(&self) -> Option<AppCmd> {
        if self.combo_2(Button::Back, Button::Start) || self.combo_2(Button::Guide, Button::Start) {
            return Some(AppCmd::TogglePause);
        }

        None
    }

    /// Generic function to check any 2-button combo
    pub fn combo_2(&self, b1: Button, b2: Button) -> bool {
        let mut state_1: Option<&ButtonState> = None;
        let mut state_2: Option<&ButtonState> = None;

        for state in &self.states {
            if state.button == b1 {
                state_1 = Some(state);
            } else if state.button == b2 {
                state_2 = Some(state);
            }
        }

        let (s1, s2) = match (state_1, state_2) {
            (Some(a), Some(b)) => (a, b),
            _ => return false,
        };

        if s1.pressed && s2.pressed {
            let diff = if s1.last_pressed > s2.last_pressed {
                s1.last_pressed.duration_since(s2.last_pressed)
            } else {
                s2.last_pressed.duration_since(s1.last_pressed)
            };

            return diff <= self.combo_interval;
        }

        false
    }
}