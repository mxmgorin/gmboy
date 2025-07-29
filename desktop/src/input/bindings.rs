use crate::app::AppCmd;
use crate::input::config::{ButtonCombo, ButtonCombos};
use sdl2::controller::Button;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub gamepad_combos: ButtonCombos,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            gamepad_combos: HashMap::from([
                (
                    ButtonCombo::new(Button::Start, Button::Back),
                    AppCmd::ToggleMenu,
                ),
                (
                    ButtonCombo::new(Button::Start, Button::Guide),
                    AppCmd::ToggleMenu,
                ),
            ])
            .into(),
        }
    }
}
