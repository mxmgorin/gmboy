use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::input::all_buttons;
use crate::input::config::InputConfig;
use core::auxiliary::joypad::JoypadButton;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
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

pub struct GamepadHandler {
    states: [ButtonState; all_buttons().len()],
}

impl GamepadHandler {
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

    pub fn handle_button(
        &mut self,
        config: &InputConfig,
        button: Button,
        is_pressed: bool,
    ) -> Option<AppCmd> {
        let cmd = self.update(button, is_pressed, config);

        if cmd.is_some() {
            return cmd;
        }

        config
            .bindings
            .buttons
            .get(button, is_pressed)
            .map(|x| x.to_owned())
    }

    pub fn handle_axis(&self, config: &InputConfig, axis_idx: u8, value: i16) -> Option<AppCmd> {
        if axis_idx == config.bindings.left_trigger.code
            && !config.bindings.left_trigger.is_pressed(value)
        {
            return config.bindings.left_trigger.cmd.clone();
        } else if axis_idx == config.bindings.right_trigger.code
            && !config.bindings.right_trigger.is_pressed(value)
        {
            return config.bindings.right_trigger.cmd.clone();
        }

        None
    }

    fn update(&mut self, button: Button, pressed: bool, config: &InputConfig) -> Option<AppCmd> {
        for state in self.states.iter_mut() {
            if state.button == button {
                state.update(pressed);
                return self.find_combo(config);
            }
        }

        None
    }

    fn find_combo(&self, config: &InputConfig) -> Option<AppCmd> {
        for combo in config.bindings.combo_buttons.iter() {
            if self.combo_2(combo.btn_1, combo.btn_2, config.combo_interval) {
                return Some(combo.cmd.to_owned());
            }
        }

        None
    }

    /// Generic function to check any 2-button combo
    fn combo_2(&self, b1: Button, b2: Button, duration: Duration) -> bool {
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

            return diff <= duration;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct ButtonBindings {
    cmds: [Option<AppCmd>; ButtonBindings::COUNT * 2],
}

impl ButtonBindings {
    const COUNT: usize = 15;

    #[inline(always)]
    fn idx(btn: Button, pressed: bool) -> usize {
        (btn as usize) * 2 + if pressed { 0 } else { 1 }
    }

    #[inline(always)]
    pub fn get(&self, btn: Button, pressed: bool) -> Option<&AppCmd> {
        self.cmds
            .get(Self::idx(btn, pressed))
            .and_then(|x| x.as_ref())
    }

    #[inline(always)]
    pub fn set(&mut self, btn: Button, pressed: bool, cmd: AppCmd) {
        self.cmds[Self::idx(btn, pressed)] = Some(cmd);
    }

    fn bind_btn(&mut self, btn: Button, joypad_btn: JoypadButton) {
        self.set(btn, true, AppCmd::PressButton(joypad_btn));
        self.set(btn, false, AppCmd::ReleaseButton(joypad_btn));
    }
}

#[derive(Clone, Debug)]
pub struct ComboButton {
    pub btn_1: Button,
    pub btn_2: Button,
    pub cmd: AppCmd,
}

impl ComboButton {
    pub fn new(btn_1: Button, btn_2: Button, cmd: AppCmd) -> Self {
        Self { btn_1, btn_2, cmd }
    }
}

impl Default for ButtonBindings {
    fn default() -> Self {
        let mut bindings = ButtonBindings {
            cmds: std::array::from_fn(|_| None),
        };

        bindings.bind_btn(Button::Start, JoypadButton::Start);
        bindings.bind_btn(Button::Guide, JoypadButton::Select);
        bindings.bind_btn(Button::Back, JoypadButton::Select);
        bindings.bind_btn(Button::DPadUp, JoypadButton::Up);
        bindings.bind_btn(Button::DPadDown, JoypadButton::Down);
        bindings.bind_btn(Button::DPadLeft, JoypadButton::Left);
        bindings.bind_btn(Button::DPadRight, JoypadButton::Right);
        bindings.bind_btn(Button::A, JoypadButton::A);
        bindings.bind_btn(Button::B, JoypadButton::B);
        bindings.set(Button::Y, true, AppCmd::ToggleRewind);
        bindings.set(Button::Y, false, AppCmd::ToggleRewind);

        bindings.set(
            Button::X,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette),
        );
        bindings.set(
            Button::LeftShoulder,
            true,
            AppCmd::ChangeMode(RunMode::Slow),
        );
        bindings.set(
            Button::LeftShoulder,
            false,
            AppCmd::ChangeMode(RunMode::Normal),
        );
        bindings.set(
            Button::RightShoulder,
            true,
            AppCmd::ChangeMode(RunMode::Turbo),
        );
        bindings.set(
            Button::RightShoulder,
            false,
            AppCmd::ChangeMode(RunMode::Normal),
        );

        bindings
    }
}

pub fn default_combo_buttons() -> Vec<ComboButton> {
    vec![
        ComboButton::new(
            Button::Back,
            Button::B,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevShader),
        ),
        ComboButton::new(
            Button::Guide,
            Button::B,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevShader),
        ),
        ComboButton::new(
            Button::Back,
            Button::A,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::NextShader),
        ),
        ComboButton::new(
            Button::Guide,
            Button::A,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::NextShader),
        ),
        ComboButton::new(Button::Start, Button::Back, AppCmd::ToggleMenu),
        ComboButton::new(Button::Start, Button::Guide, AppCmd::ToggleMenu),
        ComboButton::new(
            Button::Guide,
            Button::X,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette),
        ),
        ComboButton::new(
            Button::Back,
            Button::X,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette),
        ),
        ComboButton::new(
            Button::LeftShoulder,
            Button::Back,
            AppCmd::SaveState(SaveStateCmd::Load, None),
        ),
        ComboButton::new(
            Button::RightShoulder,
            Button::Back,
            AppCmd::SaveState(SaveStateCmd::Create, None),
        ),
        ComboButton::new(
            Button::LeftShoulder,
            Button::Guide,
            AppCmd::SaveState(SaveStateCmd::Load, None),
        ),
        ComboButton::new(
            Button::RightShoulder,
            Button::Guide,
            AppCmd::SaveState(SaveStateCmd::Create, None),
        ),
        ComboButton::new(
            Button::DPadUp,
            Button::Start,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.1)),
        ),
        ComboButton::new(
            Button::DPadDown,
            Button::Start,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.1)),
        ),
        ComboButton::new(
            Button::DPadLeft,
            Button::Start,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::DecSaveAndLoadIndexes),
        ),
        ComboButton::new(
            Button::DPadRight,
            Button::Start,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::IncSaveAndLoadIndexes),
        ),
    ]
}
