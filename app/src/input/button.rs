use crate::app::{App, AppCmd, AppState};
use core::auxiliary::joypad::JoypadButton;
use core::emu::Emu;
use crate::{PlatformFileDialog, PlatformFileSystem};

pub fn handle_emu_btn<FS, FD>(
    btn: JoypadButton,
    is_pressed: bool,
    app: &mut App<FS, FD>,
    emu: &mut Emu,
) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    match btn {
        JoypadButton::Start => emu.runtime.bus.io.joypad.start = is_pressed,
        JoypadButton::Select => emu.runtime.bus.io.joypad.select = is_pressed,
        JoypadButton::A => return handle_a(is_pressed, app, emu),
        JoypadButton::B => handle_b(is_pressed, app, emu),
        JoypadButton::Up => handle_up(is_pressed, app, emu),
        JoypadButton::Down => handle_down(is_pressed, app, emu),
        JoypadButton::Left => return handle_left(is_pressed, app, emu),
        JoypadButton::Right => return handle_right(is_pressed, app, emu),
    }

    None
}

pub fn handle_start<FS, FD>(is_pressed: bool, _app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    emu.runtime.bus.io.joypad.start = is_pressed;

    None
}

pub fn handle_select<FS, FD>(is_pressed: bool, _app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    emu.runtime.bus.io.joypad.select = is_pressed;

    None
}

pub fn handle_up<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        app.menu.move_up();
    } else {
        emu.runtime.bus.io.joypad.up = is_pressed;
    }
}

pub fn handle_down<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        app.menu.move_down();
    } else {
        emu.runtime.bus.io.joypad.down = is_pressed;
    }
}

pub fn handle_left<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        return app.menu.move_left(&app.config);
    } else {
        emu.runtime.bus.io.joypad.left = is_pressed;
    }

    None
}

pub fn handle_right<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        return app.menu.move_right(&app.config);
    } else {
        emu.runtime.bus.io.joypad.right = is_pressed
    }

    None
}

pub fn handle_a<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where 
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        return app.menu.select(&app.config, &app.platform.fs, &app.roms);
    } else {
        emu.runtime.bus.io.joypad.a = is_pressed;
    }

    None
}

pub fn handle_b<FS, FD>(is_pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog
{
    if app.state == AppState::Paused && is_pressed {
        app.menu.back();
    } else {
        emu.runtime.bus.io.joypad.b = is_pressed;
    }
}
