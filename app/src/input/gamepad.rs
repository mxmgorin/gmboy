use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::input::combo::ComboHandler;
use crate::input::config::InputConfig;
use core::auxiliary::joypad::JoypadButton;
use core::emu::runtime::RunMode;
use sdl2::controller::Button;

pub struct GamepadHandler {
    combo_handler: ComboHandler,
}

impl GamepadHandler {
    pub fn new() -> Self {
        Self {
            combo_handler: ComboHandler::new(),
        }
    }

    pub fn handle_button(
        &mut self,
        config: &InputConfig,
        button: Button,
        is_pressed: bool,
    ) -> Option<AppCmd> {
        let cmd = self.combo_handler.handle(button, is_pressed, config);

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
