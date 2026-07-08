//! Browser (WASM) frontend for GMBoy.
//!
//! This crate is a thin shell around `core`: it constructs an [`Emu`], feeds it a
//! ROM, and steps it exactly one frame at a time. The browser drives the pace via
//! `requestAnimationFrame`, so — unlike the SDL desktop frontend — we call
//! [`EmuRuntime::run_frame`] directly and never let the emulator sleep/spin itself.
//!
//! Scope: video + input + audio. The APU's samples are buffered by [`AudioSink`]
//! each frame and drained by JS via [`GmBoy::take_audio`] into a WebAudio scheduler.

use core::apu::apu::ApuConfig;
use core::apu::{Apu, SAMPLING_FREQUENCY};
use core::auxiliary::io::Io;
use core::auxiliary::joypad::JoypadButton;
use core::bus::Bus;
use core::cart::Cart;
use core::emu::config::{EmuConfig, GbModel};
use core::emu::runtime::EmuRuntime;
use core::emu::{Emu, EmuAudioCallback};
use core::ppu::lcd::Lcd;
use core::ppu::tile::PixelColor;
use core::ppu::{Ppu, LCD_X_RES, LCD_Y_RES};
use wasm_bindgen::prelude::*;

/// Accumulates the APU's interleaved stereo samples produced during a frame.
/// Drained once per frame by [`GmBoy::take_audio`].
#[derive(Default)]
struct AudioSink {
    samples: Vec<f32>,
}

impl EmuAudioCallback for AudioSink {
    fn update(&mut self, output: &[f32], _runtime: &EmuRuntime) {
        self.samples.extend_from_slice(output);
    }
}

#[wasm_bindgen]
pub struct GmBoy {
    emu: Emu,
    audio: AudioSink,
}

#[wasm_bindgen]
impl GmBoy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GmBoy {
        // Surface Rust panics in the browser console instead of an opaque trap.
        console_error_panic_hook::set_once();

        // Classic DMG green palette (RGBA hex).
        let colors = [
            PixelColor::from_hex_rgba("E0F8D0FF"),
            PixelColor::from_hex_rgba("88C070FF"),
            PixelColor::from_hex_rgba("346856FF"),
            PixelColor::from_hex_rgba("081820FF"),
        ];
        let lcd = Lcd::new(colors, GbModel::default());
        let ppu = Ppu::new(lcd);
        let apu = Apu::new(ApuConfig::default());
        let config = EmuConfig::default();
        let bus = Bus::new(Cart::empty(), Io::new(ppu, apu), config.model);
        let emu = Emu::new(config, EmuRuntime::new(bus)).expect("failed to build emu");

        GmBoy {
            emu,
            audio: AudioSink::default(),
        }
    }

    /// APU output sample rate in Hz (interleaved stereo).
    pub fn sample_rate(&self) -> u32 {
        SAMPLING_FREQUENCY
    }

    /// Native LCD width in pixels (160).
    pub fn width(&self) -> u32 {
        LCD_X_RES as u32
    }

    /// Native LCD height in pixels (144).
    pub fn height(&self) -> u32 {
        LCD_Y_RES as u32
    }

    /// Loads a ROM from raw bytes. Returns an error string for unsupported carts.
    pub fn load_rom(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        let cart = Cart::new(bytes.to_vec().into_boxed_slice()).map_err(|e| JsValue::from_str(&e))?;
        self.emu.load_cart(cart);
        Ok(())
    }

    /// Advances emulation by exactly one frame. No internal sleep — the browser
    /// paces us through `requestAnimationFrame`.
    pub fn run_frame(&mut self) {
        self.emu.runtime.run_frame(&mut self.audio);
    }

    /// Returns the current frame as RGBA bytes (`width * height * 4`), ready to be
    /// wrapped in an `ImageData` and blitted to a canvas.
    pub fn frame_buffer(&mut self) -> Vec<u8> {
        let rgb = self.emu.get_framebuffer().rgb888();
        let mut rgba = Vec::with_capacity(rgb.len() / 3 * 4);
        for px in rgb.chunks_exact(3) {
            rgba.extend_from_slice(px);
            rgba.push(0xFF);
        }
        rgba
    }

    /// Drains and returns the interleaved stereo samples (L, R, L, R, …) the APU
    /// produced since the last call. Feed these to a WebAudio scheduler.
    pub fn take_audio(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.audio.samples)
    }

    /// Sets a Game Boy button state. `name` is one of:
    /// `a`, `b`, `start`, `select`, `up`, `down`, `left`, `right`.
    pub fn set_button(&mut self, name: &str, pressed: bool) {
        let button = match name {
            "a" => JoypadButton::A,
            "b" => JoypadButton::B,
            "start" => JoypadButton::Start,
            "select" => JoypadButton::Select,
            "up" => JoypadButton::Up,
            "down" => JoypadButton::Down,
            "left" => JoypadButton::Left,
            "right" => JoypadButton::Right,
            _ => return,
        };
        self.emu
            .runtime
            .cpu
            .clock
            .bus
            .io
            .joypad
            .handle(button, pressed);
    }
}

impl Default for GmBoy {
    fn default() -> Self {
        Self::new()
    }
}
