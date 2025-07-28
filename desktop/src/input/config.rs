use crate::app::AppCmd;
use sdl2::controller::Button;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputConfig {
    pub combos: ButtonCombos,
    pub combo_interval: Duration,
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

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            combos: HashMap::from([
                (
                    ButtonCombo::new(Button::Start, Button::Back),
                    AppCmd::TogglePause,
                ),
                (
                    ButtonCombo::new(Button::Start, Button::Guide),
                    AppCmd::TogglePause,
                ),
            ])
            .into(),
            combo_interval: Duration::from_millis(500),
        }
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
        let b1 = parts
            .next()
            .and_then(|p| p.parse::<i32>().ok())
            .ok_or_else(|| serde::de::Error::custom("Invalid ButtonCombo"))?;
        let b2 = parts
            .next()
            .and_then(|p| p.parse::<i32>().ok())
            .ok_or_else(|| serde::de::Error::custom("Invalid ButtonCombo"))?;
        Ok(ButtonCombo { b1, b2 })
    }
}
