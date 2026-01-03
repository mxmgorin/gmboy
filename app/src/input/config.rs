use crate::app::AppCmd;
use crate::input::combo::{ComboButton, ComboButtonBindings};
use crate::input::gamepad::ButtonBindings;
use crate::input::keyboard::KeyBindings;
use crate::input::{all_buttons, button_to_str, str_to_button};
use sdl2::keyboard::Scancode;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub buttons: ButtonBindings,
    pub left_trigger: TriggerButtonBinding,
    pub right_trigger: TriggerButtonBinding,
    pub combo_buttons: ComboButtonBindings,
    pub keys: KeyBindings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerButtonBinding {
    pub cmd: Option<AppCmd>,
    pub code: u8,
    pub threshold: i16,
}

impl TriggerButtonBinding {
    pub fn new(cmd: Option<AppCmd>, code: u8) -> Self {
        Self {
            cmd,
            code,
            threshold: 30_000,
        }
    }

    pub fn is_pressed(&self, v: i16) -> bool {
        v > self.threshold
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            buttons: ButtonBindings::default(),
            keys: KeyBindings::default(),
            left_trigger: TriggerButtonBinding::new(None, 2),
            right_trigger: TriggerButtonBinding::new(None, 5),
            combo_buttons: ComboButtonBindings::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputConfig {
    #[serde(with = "bindings_file")]
    pub bindings: Bindings,
    pub combo_interval: Duration,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            bindings: Bindings::default(),
            combo_interval: Duration::from_millis(500),
        }
    }
}

mod bindings_file {
    use super::*;
    use crate::get_base_dir;
    use serde::{Deserializer, Serializer};

    const FILE: &str = "bindings.json";

    pub fn serialize<S>(bindings: &Bindings, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let path = get_base_dir().join(FILE);
        core::save_json_file(&path, bindings)
            .map_err(|_| serde::ser::Error::custom("Failed to save bindings.json"))?;
        serializer.serialize_str(FILE)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Bindings, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = get_base_dir().join(FILE);
        let _path: String = String::deserialize(deserializer)?;
        core::read_json_file(path)
            .map_err(|_| serde::de::Error::custom("Failed to read bindings.json"))
    }
}

impl Serialize for ComboButton {
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

impl<'de> Deserialize<'de> for ComboButton {
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

        Ok(ComboButton {
            btn_1: b1,
            btn_2: b2,
            cmd: helper.cmd,
        })
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
        struct ButtonBindingsVisitor;

        impl<'de> Visitor<'de> for ButtonBindingsVisitor {
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
                let mut bindings = ButtonBindings::default();

                while let Some((key, cmd)) = access.next_entry::<String, AppCmd>()? {
                    let mut parts = key.split('.');
                    let btn_str = parts.next().unwrap_or("");
                    let state_str = parts.next().unwrap_or("pressed");

                    let pressed = match state_str {
                        "pressed" => true,
                        "released" => false,
                        _ => return Err(M::Error::custom(format!("Invalid state in key: {key}"))),
                    };

                    if let Some(btn) = str_to_button(btn_str) {
                        bindings.set(btn, pressed, cmd);
                    } else {
                        return Err(M::Error::custom(format!("Unknown button: {btn_str}")));
                    }
                }

                Ok(bindings)
            }
        }

        deserializer.deserialize_map(ButtonBindingsVisitor)
    }
}

impl Serialize for KeyBindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_ser = serializer.serialize_map(None)?;

        for (sc, pressed, cmd) in self.iter() {
            let key = format!(
                "{}.{}",
                sc.name(),
                if pressed { "pressed" } else { "released" }
            );
            map_ser.serialize_entry(&key, cmd)?;
        }

        map_ser.end()
    }
}

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyBindingsVisitor;

        impl<'de> Visitor<'de> for KeyBindingsVisitor {
            type Value = KeyBindings;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a map of keyboard key.state (pressed/released) to AppCmd"
                )
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut bindings = KeyBindings::default();

                while let Some((key, cmd)) = access.next_entry::<String, AppCmd>()? {
                    let mut parts = key.split('.');
                    let sc_str = parts.next().unwrap_or("");
                    let state_str = parts.next().unwrap_or("pressed");

                    let pressed = match state_str {
                        "pressed" => true,
                        "released" => false,
                        _ => return Err(M::Error::custom(format!("Invalid state in key: {key}"))),
                    };

                    if let Some(sc) = Scancode::from_name(sc_str) {
                        bindings.bind_cmd(sc, pressed, cmd);
                    } else {
                        return Err(M::Error::custom(format!("Unknown key: {sc_str}")));
                    }
                }

                Ok(bindings)
            }
        }

        deserializer.deserialize_map(KeyBindingsVisitor)
    }
}
