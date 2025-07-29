use crate::app::AppCmd;
use crate::input::all_buttons;
use crate::input::config::InputConfig;
use sdl2::controller::Button;
use std::time::{Duration, Instant};

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

pub struct ComboTracker {
    states: [ButtonState; all_buttons().len()],
}

impl ComboTracker {
    pub fn new() -> Self {
        Self {
            states: [
                ButtonState::new(Button::Start),
                ButtonState::new(Button::Back),
                ButtonState::new(Button::Guide),
                ButtonState::new(Button::A),
                ButtonState::new(Button::B),
                ButtonState::new(Button::X),
                ButtonState::new(Button::Y),
                ButtonState::new(Button::LeftShoulder),
                ButtonState::new(Button::RightShoulder),
                ButtonState::new(Button::DPadUp),
                ButtonState::new(Button::DPadDown),
                ButtonState::new(Button::DPadLeft),
                ButtonState::new(Button::DPadRight),
                ButtonState::new(Button::LeftStick),
                ButtonState::new(Button::RightStick),
            ],
        }
    }

    pub fn update(
        &mut self,
        button: Button,
        pressed: bool,
        config: &InputConfig,
    ) -> Option<AppCmd> {
        for state in self.states.iter_mut() {
            if state.button == button {
                state.update(pressed);
                return self.find(config);
            }
        }

        None
    }

    fn find(&self, config: &InputConfig) -> Option<AppCmd> {
        for combo in config.bindings.gamepad_combos.iter() {
            if self.combo_2(combo.btn_1, combo.btn_2, config.combo_interval) {
                return Some(combo.cmd.to_owned());
            }
        }

        None
    }

    /// Generic function to check any 2-button combo
    fn combo_2(&self, b1: i32, b2: i32, duration: Duration) -> bool {
        let mut state_1: Option<&ButtonState> = None;
        let mut state_2: Option<&ButtonState> = None;

        for state in &self.states {
            if state.button as i32 == b1 {
                state_1 = Some(state);
            } else if state.button as i32 == b2 {
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

            return diff <= duration;
        }

        false
    }
}
