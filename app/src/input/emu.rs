use crate::app::{App, AppCmd, AppState};
use crate::{PlatformFileDialog, PlatformFileSystem};
use core::auxiliary::joypad::JoypadButton;
use core::emu::Emu;

pub fn handle_emu_btn<FS, FD>(
    btn: JoypadButton,
    pressed: bool,
    app: &mut App<FS, FD>,
    emu: &mut Emu,
) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    match btn {
        JoypadButton::Start => return handle_start(pressed, app, emu),
        JoypadButton::Select => handle_select(pressed, app, emu),
        JoypadButton::A => return handle_a(pressed, app, emu),
        JoypadButton::B => handle_b(pressed, app, emu),
        JoypadButton::Up => handle_up(pressed, app, emu),
        JoypadButton::Down => handle_down(pressed, app, emu),
        JoypadButton::Left => return handle_left(pressed, app, emu),
        JoypadButton::Right => return handle_right(pressed, app, emu),
    }

    None
}

pub fn handle_up<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        app.menu.move_up();
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.up = pressed;
    }
}

pub fn handle_down<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        app.menu.move_down();
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.down = pressed;
    }
}

pub fn handle_left<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        return app.menu.move_left(&app.config);
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.left = pressed;
    }

    None
}

pub fn handle_right<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        return app.menu.move_right(&app.config);
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.right = pressed
    }

    None
}

pub fn handle_a<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        return app.menu.select(&app.config, &app.platform.fs, &app.roms);
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.a = pressed;
    }

    None
}

pub fn handle_b<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        app.menu.back();
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.b = pressed;
    }
}

pub fn handle_start<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu) -> Option<AppCmd>
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        return app.menu.select(&app.config, &app.platform.fs, &app.roms);
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.start = pressed;
    }

    None
}

pub fn handle_select<FS, FD>(pressed: bool, app: &mut App<FS, FD>, emu: &mut Emu)
where
    FS: PlatformFileSystem,
    FD: PlatformFileDialog,
{
    if app.state == AppState::Paused && pressed {
        app.menu.back();
    } else {
        emu.runtime.cpu.clock.bus.io.joypad.select = pressed;
    }
}
