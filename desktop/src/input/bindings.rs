use crate::app::AppCmd;
use sdl2::controller::Button;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bindings {
    pub gamepad_combos: Vec<ButtonCombo>,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            gamepad_combos: vec![
                ButtonCombo::new(Button::Start, Button::Back, AppCmd::ToggleMenu),
                ButtonCombo::new(Button::Start, Button::Guide, AppCmd::ToggleMenu),
            ],
        }
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
