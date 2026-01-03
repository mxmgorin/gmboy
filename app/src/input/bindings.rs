use crate::app::AppCmd;
use core::auxiliary::joypad::JoypadButton;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub trait BindableInput: Copy {
    const COUNT: usize;

    fn to_index(self) -> usize;
    fn from_index(index: usize) -> Option<Self>;
    fn name(self) -> &'static str;
    fn from_name(name: &str) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub struct InputBindings<K: BindableInput> {
    cmds: Vec<Option<AppCmd>>,
    _marker: std::marker::PhantomData<K>,
}

impl<K: BindableInput> InputBindings<K> {
    #[inline(always)]
    fn idx(sc: K, pressed: bool) -> usize {
        sc.to_index() * 2 + if pressed { 0 } else { 1 }
    }

    #[inline(always)]
    pub fn get(&self, sc: K, pressed: bool) -> Option<&AppCmd> {
        self.cmds
            .get(Self::idx(sc, pressed))
            .and_then(|x| x.as_ref())
    }

    #[inline(always)]
    pub fn bind_cmd(&mut self, sc: K, pressed: bool, cmd: AppCmd) {
        let idx = Self::idx(sc, pressed);
        self.cmds[idx] = Some(cmd);
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, bool, &AppCmd)> {
        self.cmds.iter().enumerate().filter_map(|(i, entry)| {
            let cmd = entry.as_ref()?;
            let sc_index = i / 2;
            let pressed = (i % 2) == 0;
            let sc = K::from_index(sc_index)?;

            Some((sc, pressed, cmd))
        })
    }

    pub fn bind_btn(&mut self, sc: K, btn: JoypadButton) {
        self.bind_cmd(sc, true, AppCmd::PressButton(btn));
        self.bind_cmd(sc, false, AppCmd::ReleaseButton(btn));
    }
}

impl<K: BindableInput> Default for InputBindings<K> {
    fn default() -> Self {
        Self {
            cmds: vec![None; K::COUNT * 2],
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K> Serialize for InputBindings<K>
where
    K: BindableInput,
{
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

impl<'de, K> Deserialize<'de> for InputBindings<K>
where
    K: BindableInput,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyBindingsVisitor<K: BindableInput>(std::marker::PhantomData<K>);

        impl<'de, K> Visitor<'de> for KeyBindingsVisitor<K>
        where
            K: BindableInput,
        {
            type Value = InputBindings<K>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map of key.state (pressed/released) to AppCmd")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut bindings = InputBindings::<K>::default();

                while let Some((key, cmd)) = access.next_entry::<String, AppCmd>()? {
                    let mut parts = key.split('.');
                    let key_name = parts.next().unwrap_or("");
                    let state_str = parts.next().unwrap_or("pressed");

                    let pressed = match state_str {
                        "pressed" => true,
                        "released" => false,
                        _ => return Err(M::Error::custom(format!("Invalid state in key: {key}"))),
                    };

                    let sc = K::from_name(key_name)
                        .ok_or_else(|| M::Error::custom(format!("Unknown key: {key_name}")))?;

                    bindings.bind_cmd(sc, pressed, cmd);
                }

                Ok(bindings)
            }
        }

        deserializer.deserialize_map(KeyBindingsVisitor::<K>(std::marker::PhantomData))
    }
}
