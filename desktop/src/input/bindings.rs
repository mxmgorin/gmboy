use crate::app::{AppCmd, ChangeAppConfigCmd};
use crate::input::{all_buttons, button_to_str, str_to_button};
use core::auxiliary::joypad::JoypadButton;
use core::emu::runtime::RunMode;
use core::emu::state::SaveStateCmd;
use sdl2::controller::Button;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::option::Option;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub buttons: ButtonBindings,
    pub left_trigger: TriggerButtonConfig,
    pub right_trigger: TriggerButtonConfig,
    pub button_combos: Vec<ButtonCombo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerButtonConfig {
    pub cmd: Option<AppCmd>,
    pub code: u8,
    pub threshold: i16,
}

impl TriggerButtonConfig {
    pub fn is_pressed(&self, v: i16) -> bool {
        v > self.threshold
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            buttons: ButtonBindings::new(),
            left_trigger: TriggerButtonConfig {
                cmd: None,
                code: 2,
                threshold: 30_000,
            },
            right_trigger: TriggerButtonConfig {
                cmd: None,
                code: 5,
                threshold: 30_000,
            },
            button_combos: vec![
                ButtonCombo::new(Button::Start, Button::Back, AppCmd::ToggleMenu),
                ButtonCombo::new(Button::Start, Button::Guide, AppCmd::ToggleMenu),
                ButtonCombo::new(
                    Button::Guide,
                    Button::X,
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette),
                ),
                ButtonCombo::new(
                    Button::Back,
                    Button::X,
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::InvertPalette),
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
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(0.1)),
                ),
                ButtonCombo::new(
                    Button::DPadDown,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::Volume(-0.1)),
                ),
                ButtonCombo::new(
                    Button::DPadLeft,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::DecSaveAndLoadIndexes),
                ),
                ButtonCombo::new(
                    Button::DPadRight,
                    Button::Start,
                    AppCmd::ChangeConfig(ChangeAppConfigCmd::IncSaveAndLoadIndexes),
                ),
            ],
        }
    }
}
#[derive(Debug, Clone)]
pub struct ButtonBindings {
    map: [Option<AppCmd>; ButtonBindings::BUTTON_COUNT * 2],
}

impl ButtonBindings {
    pub const BUTTON_COUNT: usize = 15;

    #[inline(always)]
    fn idx(btn: Button, pressed: bool) -> usize {
        (btn as usize) * 2 + if pressed { 0 } else { 1 }
    }

    fn set_both(&mut self, btn: Button, cmd: AppCmd) {
        self.set(btn, true, cmd.clone());
        self.set(btn, false, cmd);
    }

    pub fn new() -> Self {
        let mut bindings = ButtonBindings {
            map: std::array::from_fn(|_| None),
        };
        bindings.set_both(Button::Start, AppCmd::EmuButton(JoypadButton::Start));
        bindings.set_both(Button::Guide, AppCmd::EmuButton(JoypadButton::Select));
        bindings.set_both(Button::Back, AppCmd::EmuButton(JoypadButton::Select));
        bindings.set_both(Button::DPadUp, AppCmd::EmuButton(JoypadButton::Up));
        bindings.set_both(Button::DPadDown, AppCmd::EmuButton(JoypadButton::Down));
        bindings.set_both(Button::DPadLeft, AppCmd::EmuButton(JoypadButton::Left));
        bindings.set_both(Button::DPadRight, AppCmd::EmuButton(JoypadButton::Right));
        bindings.set_both(Button::A, AppCmd::EmuButton(JoypadButton::A));
        bindings.set_both(Button::B, AppCmd::EmuButton(JoypadButton::B));
        bindings.set_both(Button::Y, AppCmd::ToggleRewind);
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

    #[inline(always)]
    pub fn get(&self, btn: Button, pressed: bool) -> Option<&AppCmd> {
        self.map
            .get(Self::idx(btn, pressed))
            .and_then(|x| x.as_ref())
    }

    #[inline(always)]
    pub fn set(&mut self, btn: Button, pressed: bool, action: AppCmd) {
        self.map[Self::idx(btn, pressed)] = Some(action);
    }
}

impl Serialize for ButtonBindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_ser = serializer.serialize_map(None)?;

        for btn in all_buttons() {
            for &pressed in &[true, false] {
                if let Some(cmd) = self.get(*btn, pressed) {
                    let key = format!(
                        "{}.{}",
                        button_to_str(*btn),
                        if pressed { "pressed" } else { "released" }
                    );
                    map_ser.serialize_entry(&key, cmd)?;
                }
            }
        }

        map_ser.end()
    }
}

impl<'de> Deserialize<'de> for ButtonBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BindingsVisitor;

        impl<'de> Visitor<'de> for BindingsVisitor {
            type Value = ButtonBindings;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a map of gamepad button.state (pressed/released) to AppCmd"
                )
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut bindings = ButtonBindings::new();

                while let Some((key, cmd)) = access.next_entry::<String, AppCmd>()? {
                    let mut parts = key.split('.');
                    let btn_str = parts.next().unwrap_or("");
                    let state_str = parts.next().unwrap_or("pressed");

                    let pressed = match state_str {
                        "pressed" => true,
                        "released" => false,
                        _ => {
                            return Err(M::Error::custom(format!("Invalid state in key: {}", key)))
                        }
                    };

                    if let Some(btn) = str_to_button(btn_str) {
                        bindings.set(btn, pressed, cmd);
                    } else {
                        return Err(M::Error::custom(format!("Unknown button: {}", btn_str)));
                    }
                }

                Ok(bindings)
            }
        }

        deserializer.deserialize_map(BindingsVisitor)
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

impl Serialize for ButtonCombo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ButtonCombo", 3)?;
        state.serialize_field("btn_1", &button_to_str(self.btn_1))?;
        state.serialize_field("btn_2", &button_to_str(self.btn_2))?;
        state.serialize_field("cmd", &self.cmd)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ButtonCombo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ComboHelper {
            btn_1: String,
            btn_2: String,
            cmd: AppCmd,
        }

        let helper = ComboHelper::deserialize(deserializer)?;
        let b1 = str_to_button(&helper.btn_1)
            .ok_or_else(|| D::Error::custom(format!("Unknown button: {}", helper.btn_1)))?;
        let b2 = str_to_button(&helper.btn_2)
            .ok_or_else(|| D::Error::custom(format!("Unknown button: {}", helper.btn_2)))?;

        Ok(ButtonCombo {
            btn_1: b1,
            btn_2: b2,
            cmd: helper.cmd,
        })
    }
}
