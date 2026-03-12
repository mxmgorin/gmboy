<p align="left">
  <a href="https://github.com/mxmgorin/gmboy">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01l.gif" alt="Logo" width="200">
  </a>
</p>

---

[![Tests](https://github.com/mxmgorin/gmboy/actions/workflows/test.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Android](https://github.com/mxmgorin/gmboy/actions/workflows/build-android.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Windows](https://github.com/mxmgorin/gmboy/actions/workflows/build-windows.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![MacOS](https://github.com/mxmgorin/gmboy/actions/workflows/build-macos.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Linux x86_64](https://github.com/mxmgorin/gmboy/actions/workflows/build-linux.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Linux ARM](https://github.com/mxmgorin/gmboy/actions/workflows/build-linux-arm.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![Release](https://img.shields.io/github/v/release/mxmgorin/gmboy.svg?color=blue)](https://github.com/mxmgorin/gmboy/releases)

<!--
[![Lines of code](https://tokei.rs/b1/github/mxmgorin/gmboy)](https://github.com/mxmgorin/gmboy) [![Downloads](https://img.shields.io/github/downloads/mxmgorin/gmboy/total.svg)](https://github.com/mxgorin/gmboy/releases)
[![Rust](https://img.shields.io/badge/language-Rust-blue.svg)](https://www.rust-lang.org)
[![Dependencies](https://deps.rs/repo/github/mxmgorin/gmboy/status.svg)](https://deps.rs/repo/github/mxmgorin/gmboy)
-->

<p align="center">
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" alt="Demo 1" width="200"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/02bg.gif" alt="Demo 2" width="200"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/03bg.gif" alt="Demo 3" width="200"/>
  </a>&nbsp;&nbsp;  
  <a href="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/pokemoncrystal.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/pokemoncrystal.gif" alt="Demo 4" width="200"/>
  </a>
</p>

`GMBoy` is a Game Boy and Game Boy Color emulator written in Rust, with an SDL2-based frontend for video, audio, and input. It passes the majority of widely used accuracy tests, includes a fully featured GUI, and supports multiple platforms.

Here are some highlights:

- Cross-platform: Windows, macOS, Linux, Android; SDL2 with optional OpenGL
- Modern features: save states, filters and shaders, re-bindable controls, and more
- Accuracy-focused: sub-instruction CPU timing, dot-level PPU, and cycle-synchronized systems; validated against Blargg and Mooneye test suites
- Performance-conscious: capable of running up to 10× speed on low-power ARM handhelds (tested on Allwinner H700)

***Work in progress**: while most games run correctly, some issues may still occur.*

📥 [Download the latest release](https://github.com/mxmgorin/gmboy/releases/latest)

## Accuracy & Testing

The emulator is continuously validated against community made test suites which are executed on CI via `cargo test`:

- **Blargg** – Passes all tests
- **Mooneye** – Passes most of the tests
- **Visual** - Passes the DMG-acid2, CGB-acid2, Mangen
- **SM83 JSON** – Passes all 356,000 tests

For the complete results, see [TESTS.md](./TESTS.md).

## Features

- **Save States** – Save and resume progress with multiple slots; optional auto-save on exit and startup
- **Rewind** – Configurable rewind for undoing gameplay actions
- **Speed Control** – Change emulator’s base running speed and apply Slow or Turbo modes via keys
- **Frame Blending** – Configurable blending modes to emulate LCD ghosting (e.g., flicker reduction in _Gun ZAS_)
- **Visual Filters** – Grid, subpixel, scanline, dot-matrix, and vignette
- **Rendering Backends** - SDL2 software renderer and optional OpenGL backend with shader support
- **ROM Scanning** – Automatic ROM directory scanning with menu-based launching
- **Built-in File Browser** – Load ROMs and manage directly from the UI
- **Custom Controls** – Fully rebindable inputs with support for button combinations
- **Palettes** – Multiple built-in color palettes and user extendable by editing `palettes.json`
- **GUI & Configuration** – Configuable through GUI with optional manual editing `config.json`
- **Tile Viewer** – Real-time background and sprite tile inspection (only with SDL2 rendering)

### Debugging & Testing

- **Automated Testing** – Integrated SM83 JSON tests, Blargg, and Mooneye test suites which are executed on CI via `cargo test`

### Emulation

- **CPU**: Complete Sharp LR35902 instruction set with accurate timing
- **PPU (Graphics)**: Background, window, and sprite rendering
- **APU (Audio)**: All 4 audio channels (Square 1 & 2, Wave, Noise)
- **Cartridge MBCs**: MBC0, MBC1, MBC1M, MBC2, MBC3, MBC5
- **Battery-backed SRAM**: Persistent save data

## Default controls

| Action                           | ⌨️ Keyboard              | 🎮 Gamepad                                 |
| -------------------------------- | ------------------------ | ------------------------------------------ |
| D-pad Up                         | Arrow Up                 | D-pad Up                                   |
| D-pad Down                       | Arrow Down               | D-pad Down                                 |
| D-pad Left                       | Arrow Left               | D-pad Left                                 |
| D-pad Right                      | Arrow Right              | D-pad Right                                |
| B                                | Z                        | B                                          |
| A                                | X                        | A                                          |
| Start                            | Enter or S               | Start                                      |
| Select                           | Backspace or A           | Select                                     |
| Rewind (hold)                    | R                        | Y                                          |
| Turbo mode (hold)                | Tab                      | RB                                         |
| Slow mode (hold)                 | Space                    | LB                                         |
| Main menu                        | Esc or Q                 | Select + Start                             |
| Screen scale Up and Down         | + (Equals) and - (Minus) |                                            |
| Fullscreen Toggle                | F11                      |                                            |
| Mute audio                       | M                        |                                            |
| Invert palette                   | I                        | Select + X                                 |
| Next palette                     | P                        | X                                          |
| Load save state (1–4)            | F1–F4                    | RT or Select + RB                          |
| Create save state (1–9)          | 1–9                      | LT or Select + LB                          |
| Volume Up and Down               | PageUp and PageDown      | Start + D-pad Up and Start + D-pad Down    |
| Prev and Next Save State Slot    |                          | Start + D-pad Right and Start + D-pad Left |
| Prev and Next Shader             | [ and ]                  | Select + B and Select + A                  |
| Pause/Stepping mode              | F5                       |                                            |
| Step frame                       | F6                       |                                            |
| Step scanline                    | F7                       |                                            |
| Clear screen                     | F10                      |                                            |
| Toggle debugger (In debug build) | ~                        |                                            |

## 🛠️ Building

First, make sure you have Rust installed. If you don't, install it with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, install dependencies:

Arch Linux:

```bash
sudo pacman -S sdl2
```

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
- [DMG acid2 test](https://github.com/mattcurrie/dmg-acid2) - PPU testing for DMG
- [CGB acid2 test](https://github.com/mattcurrie/cgb-acid2) - PPU testing for CGB
- [MagenTests](https://github.com/alloncm/MagenTests) - PPU testing for DMG and CGB
- [Game Boy test roms](https://github.com/c-sp/game-boy-test-roms) - various test roms
- [SameBoy](https://github.com/LIJI32/SameBoy) - shaders (modified for compatibility with GLES)
