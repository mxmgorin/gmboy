use std::fmt;
use crate::app::AppCmd;
use sdl2::controller::Button;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::option::Option;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use crate::input::{all_buttons, button_to_str, str_to_button};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub gamepad: GamepadBindings,
    pub gamepad_combos: Vec<ButtonCombo>,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            gamepad: GamepadBindings::new(),
            gamepad_combos: vec![
                ButtonCombo::new(Button::Start, Button::Back, AppCmd::ToggleMenu),
                ButtonCombo::new(Button::Start, Button::Guide, AppCmd::ToggleMenu),
            ],
        }
    }
}
#[derive(Debug, Clone)]
pub struct GamepadBindings {
    map: [Option<AppCmd>; GamepadBindings::BUTTON_COUNT],
}

impl GamepadBindings {
    pub const BUTTON_COUNT: usize = 15;

    #[inline(always)]
    fn idx(btn: Button) -> usize {
        btn as usize
    }

    pub fn new() -> Self {
        Self { map: std::array::from_fn(|_| None) }
    }

    #[inline(always)]
    pub fn get(&self, btn: Button) -> Option<&AppCmd> {
        self.map.get(Self::idx(btn)).and_then(|x| x.as_ref())
    }

    #[inline(always)]
    pub fn set(&mut self, btn: Button, action: AppCmd) {
        self.map[Self::idx(btn)] = Some(action);
    }
}

impl Serialize for GamepadBindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_ser = serializer.serialize_map(None)?;
        for btn in all_buttons() {
            if let Some(cmd) = self.get(*btn) {
                map_ser.serialize_entry(&button_to_str(*btn), &cmd)?;
            }
        }

        map_ser.end()
    }
}

impl<'de> Deserialize<'de> for GamepadBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BindingsVisitor;

        impl<'de> Visitor<'de> for BindingsVisitor {
            type Value = GamepadBindings;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map of gamepad button names to AppCmd")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut bindings = GamepadBindings::new();

                while let Some((btn_name, cmd)) = access.next_entry::<String, AppCmd>()? {
                    if let Some(btn) = str_to_button(&btn_name) {
                        bindings.set(btn, cmd);
                    } else {
                        return Err(M::Error::custom(format!("Unknown button: {}", btn_name)));
                    }
                }

                Ok(bindings)
            }
        }

        deserializer.deserialize_map(BindingsVisitor)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ButtonCombo {
    pub btn_1: i32,
    pub btn_2: i32,
    pub cmd: AppCmd,
}

impl ButtonCombo {
    pub fn new(b1: Button, b2: Button, cmd: AppCmd) -> Self {
        Self {
            btn_1: b1 as i32,
            btn_2: b2 as i32,
            cmd,
        }
    }
}
