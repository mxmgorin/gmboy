use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::input::bindings::{BindableInput, InputBindings};
use crate::input::combo::ComboHandler;
use crate::input::config::InputConfig;
use crate::input::{button_to_str, str_to_button};
use core::auxiliary::joypad::JoypadButton;
use core::emu::runtime::RunMode;
use sdl2::controller::Button;

impl BindableInput for Button {
    const COUNT: usize = 15;

    #[inline(always)]
    fn to_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Button::A),
            1 => Some(Button::B),
            2 => Some(Button::X),
            3 => Some(Button::Y),
            4 => Some(Button::Back),
            5 => Some(Button::Guide),
            6 => Some(Button::Start),
            7 => Some(Button::LeftStick),
            8 => Some(Button::RightStick),
            9 => Some(Button::LeftShoulder),
            10 => Some(Button::RightShoulder),
            11 => Some(Button::DPadUp),
            12 => Some(Button::DPadDown),
            13 => Some(Button::DPadLeft),
            14 => Some(Button::DPadRight),
            15 => Some(Button::Misc1),
            16 => Some(Button::Paddle1),
            17 => Some(Button::Paddle2),
            18 => Some(Button::Paddle3),
            19 => Some(Button::Paddle4),
            20 => Some(Button::Touchpad),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        button_to_str(self)
    }

    fn from_name(name: &str) -> Option<Self> {
        str_to_button(name)
    }
}

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

pub fn default_buttons() -> InputBindings<Button> {
    let mut bindings = InputBindings::<Button>::default();

    bindings.bind_btn(Button::Start, JoypadButton::Start);
    bindings.bind_btn(Button::Guide, JoypadButton::Select);
    bindings.bind_btn(Button::Back, JoypadButton::Select);
    bindings.bind_btn(Button::DPadUp, JoypadButton::Up);
    bindings.bind_btn(Button::DPadDown, JoypadButton::Down);
    bindings.bind_btn(Button::DPadLeft, JoypadButton::Left);
    bindings.bind_btn(Button::DPadRight, JoypadButton::Right);
    bindings.bind_btn(Button::A, JoypadButton::A);
    bindings.bind_btn(Button::B, JoypadButton::B);
    bindings.bind_cmd(Button::Y, true, AppCmd::ToggleRewind);
    bindings.bind_cmd(Button::Y, false, AppCmd::ToggleRewind);

    bindings.bind_cmd(
        Button::X,
        true,
        AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette),
    );
    bindings.bind_cmd(
        Button::LeftShoulder,
        true,
        AppCmd::ChangeMode(RunMode::Slow),
    );
    bindings.bind_cmd(
        Button::LeftShoulder,
        false,
        AppCmd::ChangeMode(RunMode::Normal),
    );
    bindings.bind_cmd(
        Button::RightShoulder,
        true,
        AppCmd::ChangeMode(RunMode::Turbo),
    );
    bindings.bind_cmd(
        Button::RightShoulder,
        false,
        AppCmd::ChangeMode(RunMode::Normal),
    );

    bindings
}
