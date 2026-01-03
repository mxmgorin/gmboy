use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::input::config::InputConfig;
use core::auxiliary::joypad::JoypadButton;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::keyboard::Scancode;

pub fn handle_key(config: &InputConfig, sc: Scancode, pressed: bool) -> Option<AppCmd> {
    config.bindings.keys.get(sc, pressed).map(|x| x.to_owned())
}

#[derive(Debug, Clone)]
pub struct KeyBindings {
    cmds: Vec<Option<AppCmd>>,
}

impl KeyBindings {
    pub const COUNT: usize = Scancode::Num as usize;

    #[inline(always)]
    fn idx(sc: Scancode, pressed: bool) -> usize {
        (sc as usize) * 2 + if pressed { 0 } else { 1 }
    }

    #[inline(always)]
    pub fn get(&self, sc: Scancode, pressed: bool) -> Option<&AppCmd> {
        self.cmds
            .get(Self::idx(sc, pressed))
            .and_then(|x| x.as_ref())
    }

    #[inline(always)]
    pub fn bind_cmd(&mut self, sc: Scancode, pressed: bool, cmd: AppCmd) {
        self.cmds[Self::idx(sc, pressed)] = Some(cmd);
    }

    pub fn iter(&self) -> impl Iterator<Item = (Scancode, bool, &AppCmd)> {
        self.cmds.iter().enumerate().filter_map(|(i, entry)| {
            let cmd = entry.as_ref()?;
            let sc_index = i / 2;
            let pressed = (i % 2) == 0;
            let sc = Scancode::from_i32(sc_index as i32).expect("only valid scancodes");

            Some((sc, pressed, cmd))
        })
    }

    fn bind_btn(&mut self, sc: Scancode, btn: JoypadButton) {
        self.bind_cmd(sc, true, AppCmd::PressButton(btn));
        self.bind_cmd(sc, false, AppCmd::ReleaseButton(btn));
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = KeyBindings {
            cmds: vec![None; Self::COUNT * 2],
        };

        bindings.bind_btn(Scancode::Up, JoypadButton::Up);
        bindings.bind_btn(Scancode::Down, JoypadButton::Down);
        bindings.bind_btn(Scancode::Left, JoypadButton::Left);
        bindings.bind_btn(Scancode::Right, JoypadButton::Right);
        bindings.bind_btn(Scancode::Z, JoypadButton::B);
        bindings.bind_btn(Scancode::X, JoypadButton::A);
        bindings.bind_btn(Scancode::Backspace, JoypadButton::Select);
        bindings.bind_btn(Scancode::A, JoypadButton::Select);
        bindings.bind_btn(Scancode::Return, JoypadButton::Start);
        bindings.bind_btn(Scancode::Return2, JoypadButton::Start);
        bindings.bind_btn(Scancode::S, JoypadButton::Start);

        // Run mode controls
        bindings.bind_cmd(Scancode::R, true, AppCmd::ToggleRewind);
        bindings.bind_cmd(Scancode::R, false, AppCmd::ChangeMode(RunMode::Normal));

        bindings.bind_cmd(Scancode::Tab, true, AppCmd::ChangeMode(RunMode::Turbo));
        bindings.bind_cmd(Scancode::Tab, false, AppCmd::ChangeMode(RunMode::Normal));

        bindings.bind_cmd(Scancode::Space, true, AppCmd::ChangeMode(RunMode::Slow));
        bindings.bind_cmd(Scancode::Space, false, AppCmd::ChangeMode(RunMode::Normal));

        // Menu / quit
        bindings.bind_cmd(Scancode::Escape, true, AppCmd::ToggleMenu);
        bindings.bind_cmd(Scancode::Q, true, AppCmd::ToggleMenu);
        // Android back key equivalent, if supported
        bindings.bind_cmd(Scancode::AcBack, true, AppCmd::ToggleMenu);

        // Scaling
        bindings.bind_cmd(
            Scancode::Equals,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(1.0)),
        );
        bindings.bind_cmd(
            Scancode::Minus,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Scale(-1.0)),
        );

        // Audio / video toggles
        bindings.bind_cmd(
            Scancode::M,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::ToggleMute),
        );
        bindings.bind_cmd(
            Scancode::I,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette),
        );

        // Fullscreen (side-effect only)
        bindings.bind_cmd(Scancode::F10, true, AppCmd::ToggleFullscreen);

        // Volume
        bindings.bind_cmd(
            Scancode::F11,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.05)),
        );
        bindings.bind_cmd(
            Scancode::F12,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.05)),
        );

        // Shaders / palettes
        bindings.bind_cmd(
            Scancode::LeftBracket,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::PrevShader),
        );
        bindings.bind_cmd(
            Scancode::RightBracket,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::NextShader),
        );
        bindings.bind_cmd(
            Scancode::P,
            true,
            AppCmd::ChangeConfig(ChangeAppConfigCmd::NextPalette),
        );

        // Save state – create
        bindings.bind_cmd(
            Scancode::Num1,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(1)),
        );
        bindings.bind_cmd(
            Scancode::Num2,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(2)),
        );
        bindings.bind_cmd(
            Scancode::Num3,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(3)),
        );
        bindings.bind_cmd(
            Scancode::Num4,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(4)),
        );
        bindings.bind_cmd(
            Scancode::Num5,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(5)),
        );
        bindings.bind_cmd(
            Scancode::Num6,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(6)),
        );
        bindings.bind_cmd(
            Scancode::Num7,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(7)),
        );
        bindings.bind_cmd(
            Scancode::Num8,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(8)),
        );
        bindings.bind_cmd(
            Scancode::Num9,
            true,
            AppCmd::SaveState(SaveStateCmd::Create, Some(9)),
        );

        // Save state – load
        bindings.bind_cmd(
            Scancode::F1,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(1)),
        );
        bindings.bind_cmd(
            Scancode::F2,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(2)),
        );
        bindings.bind_cmd(
            Scancode::F3,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(3)),
        );
        bindings.bind_cmd(
            Scancode::F4,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(4)),
        );
        bindings.bind_cmd(
            Scancode::F5,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(5)),
        );
        bindings.bind_cmd(
            Scancode::F6,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(6)),
        );
        bindings.bind_cmd(
            Scancode::F7,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(7)),
        );
        bindings.bind_cmd(
            Scancode::F8,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(8)),
        );
        bindings.bind_cmd(
            Scancode::F9,
            true,
            AppCmd::SaveState(SaveStateCmd::Load, Some(9)),
        );

        bindings
    }
}
