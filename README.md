<p align="left">
  <a href="https://github.com/mxmgorin/gmboy">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01l.gif" alt="Logo" width="200">
  </a>
</p>

___

[![CI](https://github.com/mxmgorin/gmboy/actions/workflows/test.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Dependencies](https://deps.rs/repo/github/mxmgorin/gmboy/status.svg)](https://deps.rs/repo/github/mxmgorin/gmboy)
<!--[![Rust](https://img.shields.io/badge/language-Rust-blue.svg)](https://www.rust-lang.org)-->
![Android](https://img.shields.io/badge/Android-blue?logo=android)
![Windows](https://img.shields.io/badge/Windows-blue?logo=windows)
![Mac](https://img.shields.io/badge/Mac-blue?logo=apple)
![Linux](https://img.shields.io/badge/Linux-blue?logo=linux)
[![GitHub release](https://img.shields.io/github/v/release/mxmgorin/gmboy.svg?color=blue)](https://github.com/mxmgorin/gmboy/releases)

<!--
[![Lines of code](https://tokei.rs/b1/github/mxmgorin/gmboy)](https://github.com/mxmgorin/gmboy) [![Downloads](https://img.shields.io/github/downloads/mxmgorin/gmboy/total.svg)](https://github.com/mxgorin/gmboy/releases) -->

<p align="center">
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" alt="Demo 1" width="260"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/02bg.gif" alt="Demo 2" width="260"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/03bg.gif" alt="Demo 3" width="260"/>
  </a>
</p>

`GMBoy` is a Game Boy emulator written in Rust. What began as an exploratory project has evolved into a more ambitious effort.

The goal of the project is to provide a well-engineered emulator that balances accuracy and performance while incorporating modern features and long-term maintainability.

Here are some highlights:

- Cross-platform: Windows, macOS, Linux, Android; SDL2 with optional OpenGL
- Modern features: save states, filters and shaders, re-bindable controls, and more
- Accuracy-focused: sub-instruction CPU timing, dot-based PPU, and synchronized components; validated against Blargg and Mooneye test suites
- Performance-conscious: capable of running up to 10× speed on low-power ARM handhelds (tested on H700)

***Work in progress**: while most games run correctly, some issues may still occur.*

📥 [Download the latest release](https://github.com/mxmgorin/gmboy/releases/latest)

## Features

### Gameplay
- **Save States** – Save and resume progress with multiple slots; optional auto-save on exit and startup
- **Rewind** – Configurable rewind for undoing gameplay actions
- **Speed Control** – Configure the emulator’s base running speed and dynamically apply Slow or Turbo modes to temporarily decrease or increase it. All speed changes are available via settings and hotkeys.

### Video & Rendering
- **Frame Blending** – Configurable blending modes to emulate LCD ghosting (e.g., flicker reduction in *Gun ZAS*)
- **Visual Filters** – Grid, subpixel, scanline, dot-matrix, and vignette
- **Rendering Backends** - SDL2 software renderer and optional OpenGL backend with shader support

### Interface & Controls
- **ROM Scanning** – Automatic ROM directory scanning with menu-based launching
- **Built-in File Browser** – Load ROMs and manage directly from the UI
- **Custom Controls** – Fully rebindable inputs with support for button combinations
- **Palettes** – Multiple built-in color palettes and user extendable by editing `palettes.json`
- **GUI & Configuration** – Configuable through GUI with optional manual editing `config.json`

### Debugging & Testing
- **Automated Testing** – Integrated SM83 JSON tests, Blargg, and Mooneye test suites which are executed on CI via `cargo test`
- **Tile Viewer** – Real-time background and sprite tile inspection

### Emulation

- **CPU**: Complete Sharp LR35902 instruction set with accurate timing
- **PPU (Graphics)**: Background, window, and sprite rendering
- **APU (Audio)**: All 4 audio channels (Square 1 & 2, Wave, Noise)
- **Cartridge MBCs**: MBC0, MBC1, MBC1M, MBC2, MBC3, MBC5
- **Battery-backed SRAM**: Persistent save data
- **Input**: Full Game Boy button support (D-Pad, A, B, Start, Select)

## Accuracy & Testing

The emulator is continuously validated against comunity made test suites:
- **SM83 JSON Tests** – Passes all 356,000 tests  
- **Blargg Tests** – Passes all tests
- **Mooneye Acceptance Tests** – Passes most of the tests

For the complete, up-to-date results, see [TESTS.md](./TESTS.md).

## Default controls

| Action                        | ⌨️ Keyboard              | 🎮 Gamepad                                  |
|-------------------------------|--------------------------|---------------------------------------------|
| D-pad Up                      | Arrow Up                 | D-pad Up                                    |
| D-pad Down                    | Arrow Down               | D-pad Down                                  |
| D-pad Left                    | Arrow Left               | D-pad Left                                  |
| D-pad Right                   | Arrow Right              | D-pad Right                                 |
| B                             | Z                        | B                                           |
| A                             | X                        | A                                           |
| Start                         | Enter or S               | Start                                       |
| Select                        | Backspace or A           | Select                                      |
| Rewind (hold)                 | R                        | Y                                           |
| Turbo mode (hold)             | Tab                      | RB                                          |
| Slow mode (hold)              | Space                    | LB                                          |
| Main menu                     | Esc or Q                 | Select + Start                              |
| Screen scale Up and Down      | + (Equals) and - (Minus) |                                             |
| Fullscreen Toggle             | F10                      |                                             |
| Mute audio                    | M                        |                                             |
| Invert palette                | I                        | Select + X                                  |
| Next palette                  | P                        | X                                           |
| Load save state (1–9)         | F1–F19                   | RT or Select + RB                           |
| Create save state (1–9)       | 1–9                      | LT or Select + LB                           |
| Volume Up and Down            | F12 and F11              | Start + D-pad Up and Start + D-pad Down     |
| Prev and Next Save State Slot |                          | Start + D-pad Right  and Start + D-pad Left |
| Prev and Next Shader          | [ and ]                  | Select + B and Select + A                   |

## 🛠️ Building

First, make sure you have Rust installed. If you don't, install it with:
````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````
Then, install dependencies:

Arch Linux:
```bash
sudo pacman -S sdl2
````

After that, you should be able to build:
```bash
cargo build --release
```

## License

This project is licensed under the terms of the **GNU General Public License v3.0 (GPLv3)**.
See the [LICENSE](LICENSE) file for the full text.

## References

Here are some useful resources for Game Boy development and emulation:

- [Game Boy Complete Technical Reference](https://gbdev.io/pandocs/)
- [Gekkio's Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
- [Game Boy CPU Opcodes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Gbops, an accurate opcode table for the Game Boy](https://izik1.github.io/gbops/index.html)
- [RGBDS GBZ80 Assembly Documentation](https://rgbds.gbdev.io/docs/v0.9.0/gbz80.7)
- [A curated list of Game Boy development resources](https://github.com/gbdev/awesome-gbdev)

## Acknowledgments

This project makes use of the following resources:

- [SM83 Tests](https://github.com/SingleStepTests/sm83) - CPU instruction tests
- [GB Test ROMs](https://github.com/retrio/gb-test-roms) - general accuracy tests
- [mooneye test suite](https://github.com/Gekkio/mooneye-test-suite) - general accuracy tests
- [DMG acid2 Test](https://github.com/mattcurrie/dmg-acid2) - PPU testing
- [SameBoy](https://github.com/LIJI32/SameBoy) - shaders (modified for compatibility with GLES)
