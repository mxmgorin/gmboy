use crate::input::bindings::Bindings;
use serde::{Deserialize, Serialize};
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
