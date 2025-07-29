use crate::app::AppCmd;
use crate::input::bindings::Bindings;
use sdl2::controller::Button;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ButtonCombo {
    pub b1: i32,
    pub b2: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonCombos(pub HashMap<ButtonCombo, AppCmd>);

impl From<HashMap<ButtonCombo, AppCmd>> for ButtonCombos {
    fn from(combos: HashMap<ButtonCombo, AppCmd>) -> Self {
        Self(combos)
    }
}

impl ButtonCombo {
    pub fn new(b1: Button, b2: Button) -> Self {
        Self {
            b1: b1 as i32,
            b2: b2 as i32,
        }
    }
}

impl Serialize for ButtonCombo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{},{}", self.b1, self.b2);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ButtonCombo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split(',');

        Ok(ButtonCombo {
            b1: parse_part::<_, D::Error>(&mut parts)?,
            b2: parse_part::<_, D::Error>(&mut parts)?,
        })
    }
}

fn parse_part<'a, I, E>(parts: &mut I) -> Result<i32, E>
where
    I: Iterator<Item = &'a str>,
    E: serde::de::Error,
{
    parts
        .next()
        .and_then(|p| p.parse::<i32>().ok())
        .ok_or_else(|| E::custom("Invalid ButtonCombo"))
}

mod bindings_file {
    use super::*;
    use serde::{Deserializer, Serializer};

    const FILE: &str = "bindings.json";

    pub fn serialize<S>(bindings: &Bindings, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let path = core::get_exe_dir().join(FILE);
        core::save_json_file(&path, bindings)
            .map_err(|_| serde::ser::Error::custom("Failed to save bindings.json"))?;
        serializer.serialize_str(FILE)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Bindings, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = core::get_exe_dir().join(FILE);
        let _path: String = String::deserialize(deserializer)?;
        core::read_json_file(path)
            .map_err(|_| serde::de::Error::custom("Failed to read bindings.json"))
    }
}
