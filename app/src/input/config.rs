use crate::app::AppCmd;
use crate::input::bindings::InputBindings;
use crate::input::combo::{ComboButton, ComboButtonBindings};
use crate::input::gamepad::default_buttons;
use crate::input::keyboard::default_keys;
use crate::input::{button_to_str, str_to_button};
use serde::de::Error;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub buttons: InputBindings<sdl2::controller::Button>,
    pub left_trigger: TriggerButtonBinding,
    pub right_trigger: TriggerButtonBinding,
    pub combo_buttons: ComboButtonBindings,
    pub keys: InputBindings<sdl2::keyboard::Scancode>,
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
            buttons: default_buttons(),
            keys: default_keys(),
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
