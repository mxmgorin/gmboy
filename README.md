<p align="center">
  <a href="https://github.com/mxmgorin/oxgbc">
    <img src="media/logo.svg" alt="oxGBC" width="200">
  </a>
</p>

<p align="center">
  <b>A Game Boy &amp; Game Boy Color emulator written in Rust.</b>
</p>

<p align="center">
  <a href="https://mxmgorin.github.io/oxgbc/"><b>🕹️&nbsp;&nbsp;Play online</b></a>
  &nbsp;&nbsp;&nbsp;
  <a href="https://github.com/mxmgorin/oxgbc/releases/latest"><b>📥&nbsp;&nbsp;Download</b></a>
</p>

---

[![Tests](https://github.com/mxmgorin/oxgbc/actions/workflows/test.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/test.yml)
[![Android](https://github.com/mxmgorin/oxgbc/actions/workflows/build-android.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/build-android.yml)
[![Windows](https://github.com/mxmgorin/oxgbc/actions/workflows/build-windows.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/build-windows.yml)
[![macOS](https://github.com/mxmgorin/oxgbc/actions/workflows/build-macos.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/build-macos.yml)
[![Linux](https://github.com/mxmgorin/oxgbc/actions/workflows/build-linux.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/build-linux.yml)
[![Linux ARM](https://github.com/mxmgorin/oxgbc/actions/workflows/build-linux-arm.yml/badge.svg)](https://github.com/mxmgorin/oxgbc/actions/workflows/build-linux-arm.yml)

[![Release](https://img.shields.io/github/v/release/mxmgorin/oxgbc?color=blue)](https://github.com/mxmgorin/oxgbc/releases/latest)
[![License](https://img.shields.io/github/license/mxmgorin/oxgbc?color=blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-blue.svg)](https://www.rust-lang.org)
[![Dependencies](https://deps.rs/repo/github/mxmgorin/oxgbc/status.svg)](https://deps.rs/repo/github/mxmgorin/oxgbc)

<!-- optional extras (re-enable Downloads once the count is higher):
[![Downloads](https://img.shields.io/github/downloads/mxmgorin/oxgbc/total.svg?color=blue)](https://github.com/mxmgorin/oxgbc/releases)
[![Lines of code](https://tokei.rs/b1/github/mxmgorin/oxgbc)](https://github.com/mxmgorin/oxgbc)
-->

<p align="center">
  <a href="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/01bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/01bg.gif" alt="Demo 1" width="200"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/02bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/02bg.gif" alt="Demo 2" width="200"/>
  </a>&nbsp;&nbsp;
  <a href="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/03bg.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/03bg.gif" alt="Demo 3" width="200"/>
  </a>&nbsp;&nbsp;  
  <a href="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/pokemoncrystal.gif" target="_blank">
    <img src="https://raw.githubusercontent.com/mxmgorin/oxgbc/main/media/pokemoncrystal.gif" alt="Demo 4" width="200"/>
  </a>
</p>

`oxGBC` (**ox**ide + **GBC**) is an accuracy-focused Game Boy and Game Boy Color emulator written in Rust. A single portable core runs [in browser](https://mxmgorin.github.io/oxgbc/) via WebAssembly, on Windows, macOS, and Linux, on Android, and fast enough for low-power ARM handhelds. It passes the majority of widely used accuracy test suites and ships with a full-featured GUI — save states, rewind, shaders, and more.

Here are some highlights:

- **Cross-platform** — one core across Windows, macOS, Linux, Android, and the web; SDL2 desktop frontend with an optional OpenGL backend
- **Accuracy-focused** — sub-instruction CPU timing, dot-level PPU, and cycle-synchronized subsystems; validated against Blargg, Mooneye, and acid2 suites in CI
- **Performance-conscious** — runs up to 10× speed on low-power ARM handhelds (tested on Allwinner H700)
- **Feature-rich** — save states, rewind, shaders and filters, palettes, rebindable controls, and a built-in tile viewer

***Work in progress**: while most games run correctly, some issues may still occur.*

The web demo bundles open-source homebrew games and test ROMs — see [ROM credits & licenses](crates/web/assets/README.md).

## Accuracy & Testing

The emulator is continuously validated against community made test suites which are executed on CI via `cargo test`:

- **Blargg** – Passes all tests
- **Mooneye** – Passes most of the tests
- **Visual** - Passes the DMG-acid2, CGB-acid2, Mangen

For the complete results, see [TESTS.md](./TESTS.md).

## Features

### Gameplay

- **Save States** – Save and resume progress with multiple slots; optional auto-save on exit and startup
- **Rewind** – Configurable rewind for undoing gameplay actions
- **Speed Control** – Change the emulator's base running speed and apply Slow or Turbo modes via keys
- **Custom Controls** – Fully rebindable inputs with support for button combinations

### Video & Rendering

- **Rendering Backends** – SDL2 software renderer and optional OpenGL backend with shader support
- **Visual Filters** – Grid, subpixel, scanline, dot-matrix, and vignette
- **Frame Blending** – Configurable blending modes to emulate LCD ghosting (e.g., flicker reduction in _Gun ZAS_)
- **Palettes** – Multiple built-in color palettes, user-extendable by editing `palettes.json`

### Interface & Tooling

- **Web Build (WASM)** – Runs entirely in the browser via WebAssembly, no install required
- **GUI & Configuration** – Configurable through the GUI, with optional manual editing of `config.json`
- **Built-in File Browser** – Load and manage ROMs directly from the UI
- **ROM Scanning** – Automatic ROM directory scanning with menu-based launching
- **Tile Viewer** – Real-time background and sprite tile inspection (SDL2 rendering only)

### Emulation

- **CPU** – Complete Sharp LR35902 instruction set with accurate timing
- **PPU (Graphics)** – Background, window, and sprite rendering
- **APU (Audio)** – All 4 audio channels (Square 1 & 2, Wave, Noise)
- **Cartridge MBCs** – MBC0, MBC1, MBC1M, MBC2, MBC3, MBC5
- **Battery-backed SRAM** – Persistent save data

## 🎮 Controls

<details>
<summary><b>Default control mappings</b> (click to expand)</summary>

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

</details>

## 📦 Installing (macOS)

Because the app is only ad-hoc signed (no paid Apple Developer ID), Gatekeeper
will refuse the first launch with an *"unidentified developer"* warning. To open
it the first time: **right-click oxGBC → Open → Open**. Alternatively, clear the
quarantine flag from a terminal:

```bash
xattr -dr com.apple.quarantine /Applications/oxGBC.app
```

## 🛠️ Building

First, make sure you have Rust installed. If you don't, install it with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install the SDL2 development libraries for your platform:

```bash
# Arch Linux
sudo pacman -S sdl2

# Debian / Ubuntu
sudo apt install libsdl2-dev

# Fedora
sudo dnf install SDL2-devel

# macOS (Homebrew)
brew install sdl2
```

> No system SDL2 (e.g. Windows)? Compile it from source with the bundled feature:
> `cargo build --release -p desktop --features sdl2-bundled`

After that, build the release binary:

```bash
cargo build --release
```

## Running

Launch with a ROM:

```bash
cargo run --release -p desktop -- path/to/game.gb
```

Or run without arguments and use the built-in file browser / ROM scanner to pick a game from the GUI:

```bash
cargo run --release -p desktop
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
