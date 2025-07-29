use sdl2::controller::Button;

pub mod bindings;
pub mod button;
pub mod combos;
pub mod config;
pub mod gamepad;
pub mod handler;
pub mod keyboard;

pub fn button_to_str(btn: Button) -> &'static str {
    match btn {
        Button::A => "A",
        Button::B => "B",
        Button::X => "X",
        Button::Y => "Y",
        Button::Back => "Back",
        Button::Guide => "Guide",
        Button::Start => "Start",
        Button::LeftStick => "LeftStick",
        Button::RightStick => "RightStick",
        Button::LeftShoulder => "LB",
        Button::RightShoulder => "RB",
        Button::DPadUp => "DPadUp",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::DPadRight => "DPadRight",
        Button::Misc1 |
        Button::Paddle1 |
        Button::Paddle2 |
        Button::Paddle3 |
        Button::Paddle4 |
        Button::Touchpad => { "" }
    }
}

pub fn str_to_button(s: &str) -> Option<Button> {
    match s {
        "A" => Some(Button::A),
        "B" => Some(Button::B),
        "X" => Some(Button::X),
        "Y" => Some(Button::Y),
        "Back" => Some(Button::Back),
        "Guide" => Some(Button::Guide),
        "Start" => Some(Button::Start),
        "LeftStick" => Some(Button::LeftStick),
        "RightStick" => Some(Button::RightStick),
        "LB" => Some(Button::LeftShoulder),
        "RB" => Some(Button::RightShoulder),
        "DPadUp" => Some(Button::DPadUp),
        "DPadDown" => Some(Button::DPadDown),
        "DPadLeft" => Some(Button::DPadLeft),
        "DPadRight" => Some(Button::DPadRight),
        _ => None,
    }
}

pub fn all_buttons() -> &'static [Button] {
    &[
        Button::A,
        Button::B,
        Button::X,
        Button::Y,
        Button::Back,
        Button::Guide,
        Button::Start,
        Button::LeftStick,
        Button::RightStick,
        Button::LeftShoulder,
        Button::RightShoulder,
        Button::DPadUp,
        Button::DPadDown,
        Button::DPadLeft,
        Button::DPadRight,
    ]
}
