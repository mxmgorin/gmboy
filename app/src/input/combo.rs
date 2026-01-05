use crate::app::{AppCmd, ChangeConfigCmd};
use crate::input::bindings::BindableInput;
use crate::input::config::{GamepadBindings, InputConfig};
use crate::input::gamepad_buttons;
use core::emu::state::SaveStateCmd;
use sdl2::controller::Button;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
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

pub struct ComboHandler {
    states: [ButtonState; Button::COUNT],
}

impl ComboHandler {
    pub fn new() -> Self {
        let mut states = [ButtonState::new(Button::A); Button::COUNT];

        for button in gamepad_buttons() {
            states[button.code()] = ButtonState::new(*button);
        }

        Self { states }
    }

    pub fn handle(
        &mut self,
        button: Button,
        pressed: bool,
        config: &InputConfig,
    ) -> Option<AppCmd> {
        let state = &mut self.states[button.code()];
        state.update(pressed);
        self.find_combo(&config.bindings.gamepad, config.combo_interval)
    }

    fn find_combo(&self, bindings: &GamepadBindings, interval: Duration) -> Option<AppCmd> {
        for combo in bindings.combo.combos.iter() {
            if self.combo_2(combo.btn_1, combo.btn_2, interval) {
                return Some(combo.cmd.to_owned());
            }
        }

        None
    }

    /// Generic function to check any 2-button combo
    fn combo_2(&self, b1: Button, b2: Button, dur: Duration) -> bool {
        let s1 = self.states[b1.code()];
        let s2 = self.states[b2.code()];

        if s1.pressed && s2.pressed {
            let diff = if s1.last_pressed > s2.last_pressed {
                s1.last_pressed.duration_since(s2.last_pressed)
            } else {
                s2.last_pressed.duration_since(s1.last_pressed)
            };

            return diff <= dur;
        }

        false
    }
}

#[derive(Clone, Debug)]
pub struct ButtonCombo {
    pub btn_1: Button,
    pub btn_2: Button,
    pub cmd: AppCmd,
}

impl ButtonCombo {
    pub fn new(btn_1: Button, btn_2: Button, cmd: AppCmd) -> Self {
        Self { btn_1, btn_2, cmd }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonComboBindings {
    combos: Vec<ButtonCombo>,
}

impl Default for ButtonComboBindings {
    fn default() -> Self {
        Self {
            combos: vec![
                ButtonCombo::new(
                    Button::Back,
                    Button::B,
                    AppCmd::ChangeConfig(ChangeConfigCmd::PrevShader),
                ),
                ButtonCombo::new(
                    Button::Guide,
                    Button::B,
                    AppCmd::ChangeConfig(ChangeConfigCmd::PrevShader),
                ),
                ButtonCombo::new(
                    Button::Back,
                    Button::A,
                    AppCmd::ChangeConfig(ChangeConfigCmd::NextShader),
                ),
                ButtonCombo::new(
                    Button::Guide,
                    Button::A,
                    AppCmd::ChangeConfig(ChangeConfigCmd::NextShader),
                ),
                ButtonCombo::new(Button::Start, Button::Back, AppCmd::ToggleMenu),
                ButtonCombo::new(Button::Start, Button::Guide, AppCmd::ToggleMenu),
                ButtonCombo::new(
                    Button::Guide,
                    Button::X,
                    AppCmd::ChangeConfig(ChangeConfigCmd::InvertPalette),
                ),
                ButtonCombo::new(
                    Button::Back,
                    Button::X,
                    AppCmd::ChangeConfig(ChangeConfigCmd::InvertPalette),
                ),
                ButtonCombo::new(
                    Button::LeftShoulder,
                    Button::Back,
                    AppCmd::SaveState(SaveStateCmd::Load, None),
                ),
                ButtonCombo::new(
                    Button::RightShoulder,
                    Button::Back,
                    AppCmd::SaveState(SaveStateCmd::Create, None),
                ),
                ButtonCombo::new(
                    Button::LeftShoulder,
                    Button::Guide,
                    AppCmd::SaveState(SaveStateCmd::Load, None),
                ),
                ButtonCombo::new(
                    Button::RightShoulder,
                    Button::Guide,
                    AppCmd::SaveState(SaveStateCmd::Create, None),
                ),
                ButtonCombo::new(
                    Button::DPadUp,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeConfigCmd::Volume(0.1)),
                ),
                ButtonCombo::new(
                    Button::DPadDown,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeConfigCmd::Volume(-0.1)),
                ),
                ButtonCombo::new(
                    Button::DPadLeft,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeConfigCmd::DecSaveAndLoadIndexes),
                ),
                ButtonCombo::new(
                    Button::DPadRight,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeConfigCmd::IncSaveAndLoadIndexes),
                ),
            ],
        }
    }
}
