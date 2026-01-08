<p align="left">
  <a href="https://github.com/mxmgorin/gmboy">
    <img src="https://raw.githubusercontent.com/mxmgorin/gmboy/main/assets/01l.gif" alt="Logo" width="200">
  </a>
</p>

___

[![CI](https://github.com/mxmgorin/gmboy/actions/workflows/test.yml/badge.svg)](https://github.com/mxmgorin/gmboy/actions)
[![GitHub release](https://img.shields.io/github/v/release/mxmgorin/gmboy.svg?color=blue)](https://github.com/mxmgorin/gmboy/releases)
[![Rust](https://img.shields.io/badge/language-Rust-blue.svg)](https://www.rust-lang.org)
![Linux](https://img.shields.io/badge/Linux-blue?logo=linux)
![Windows](https://img.shields.io/badge/Windows-blue?logo=windows)
![Mac](https://img.shields.io/badge/Mac-blue?logo=apple)
![Android](https://img.shields.io/badge/Android-blue?logo=android)
[![Dependencies](https://deps.rs/repo/github/mxmgorin/gmboy/status.svg)](https://deps.rs/repo/github/mxmgorin/gmboy)
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
- **Slow & Turbo Modes** – Adjustable emulation speed via settings or hotkeys

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
- **Automated Testing** – Integrated SM83 JSON tests, Blargg, and Mooneye test suites which are executed on CI via `cargo test`)
- **Tile Viewer** – Real-time background and sprite tile inspection

### Emulation

- **CPU**: Complete Sharp LR35902 instruction set with accurate timing
- **PPU (Graphics)**: Background, window, and sprite rendering
- **APU (Audio)**: All 4 audio channels (Square 1 & 2, Wave, Noise)
- **Cartridge MBCs**: MBC0, MBC1, MBC1M, MBC2, MBC3, MBC5
- **Battery-backed SRAM**: Persistent save data
- **Input**: Full Game Boy button support (D-Pad, A, B, Start, Select)

**Planned Features**

- JIT recompilation
- Game Boy Color (GBC) mode support
- WebAssembly builds for wider platform support
- Audio visualizer for debugging and fun audio feedback
- Custom palette loader and editor to tweak game colors
- Ongoing improvements and ongoing bug fixes

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
Then, install dependencies: **SDL2**

Arch Linux:
```bash
sudo pacman -S sdl2
````

After that, you should be able to build:
```bash
cargo build --release
```

## Test Results

- ### SM83:
Passes all of 356 000 tests ✅

- ### Blargg

| CPU Instructions        | Memory Timing          | OAM Bug                 |
| ----------------------- | ---------------------- | ----------------------- |
| 01-special.gb ✅         | 01-read\_timing.gb ✅   | 1-lcd\_sync.gb ✅        |
| 02-interrupts.gb ✅      | 02-write\_timing.gb ✅  | 2-causes.gb ✅           |
| 03-op sp,hl.gb ✅        | 03-modify\_timing.gb ✅ | 3-non\_causes.gb ✅      |
| 04-op r,imm.gb ✅        |                        | 4-scanline\_timing.gb ✅ |
| 05-op rp.gb ✅           |                        | 5-timing\_bug.gb ✅      |
| 06-ld r,r.gb ✅          |                        | 6-timing\_no\_bug.gb ✅  |
| 07-jr,jp,call,ret,rst ✅ |                        | 7-timing\_effect.gb ✅   |
| 08-misc instrs.gb ✅     |                        | 8-instr\_effect.gb ✅    |
| 09-op r,r.gb ✅          |                        |                         |
| 10-bit ops.gb ✅         |                        |                         |
| 11-op a,(hl).gb ✅       |                        |                         |


- ### Mooneye

- acceptance

| General & OAM DMA            | Timing                       | Timer Accuracy                 |
|------------------------------|------------------------------|-------------------------------|
| oam_dma/oam_dma_timing.gb ✅  | call_cc_timing.gb ✅          | div_write.gb ✅             |
| bits/mem_oam.gb ✅            | call_cc_timing2.gb ✅         | rapid_toggle.gb ✅          |
| bits/reg_f.gb ✅              | call_timing.gb ✅             | tim00.gb ✅                 |
| instr/daa.gb ✅               | call_timing2.gb ✅            | tim00_div_trigger.gb ✅     |
| oam_dma/basic.gb ✅           | div_timing.gb ✅              | tim01.gb ✅                 |
| oam_dma/reg_read.gb ✅        | ei_timing.gb ✅               | tim01_div_trigger.gb ✅     |
| oam_dma/oam_dma_restart.gb ✅ | halt_ime0_ei.gb ✅            | tim10.gb ✅                 |
| oam_dma/oam_dma_start.gb ✅   | halt_ime0_nointr_timing.gb ✅ | tim10_div_trigger.gb ✅     |
| sources-GS ✅                 | halt_ime1_timing.gb ✅        | tim11.gb ✅                 |
| unused_hwio-GS.gb ✅          | halt_ime1_timing2-GS.gb ✅    | tim11_div_trigger.gb ✅     |
| ie_push.gb ✅                | jp_cc_timing.gb ✅            | tima_reload.gb ✅           |
|                              | jp_timing.gb ✅               | tima_write_reloading.gb ✅  |
|                              | ld_hl_sp_e_timing.gb ✅       | tma_write_reloading.gb ✅   |
|                              | pop_timing.gb ✅              |                               |
|                              | push_timing.gb ✅             |                               |
|                              | ret_cc_timing.gb ✅           |                               |
|                              | ret_timing.gb ✅              |                               |
|                              | reti_intr_timing.gb ✅        |                               |
|                              | reti_timing.gb ✅             |                               |
|                              | rst_timing.gb ✅              |                               |
|                              |  add_sp_e_timing.gb ✅        |                               |
|                              | di_timing-GS.gb ✅            |                               |
|                              | intr_timing ✅                |                               |

- emulator-only

| mbc1                         | mbc2              | mbc5               |
|------------------------------|-------------------|--------------------|
| bits_bank1.gb ✅              | bits_ramg.gb ✅    | rom_512kb.gb ✅     |
| bits_bank2.gb ✅              | bits_romb.gb ✅    | rom_1Mb.gb ✅       |
| bits_mode.gb ✅               | bits_unused.gb ✅  | rom_2Mb.gb ✅       |
| bits_ramg.gb ✅               | ram.gb ✅          | rom_4Mb.gb ✅       |
| multicart_rom_8Mb.gb ✅      | rom_1Mb.gb ✅      | rom_8Mb.gb ✅       |
| ram_64kb.gb ✅               | rom_2Mb.gb ✅      | rom_16Mb.gb ✅      |
| ram_256kb.gb ✅              | rom_512kb.gb ✅    | rom_32Mb.gb ✅      |
| rom_1Mb.gb ✅                |                   |                    |
| rom_2Mb.gb ✅                |                   |                    |
| rom_4Mb.gb ✅                |                   |                    |
| rom_8Mb.gb ✅                |                   |                    |
| rom_16Mb.gb ✅               |                   |                    |
| rom_512kb.gb ✅              |                   |                    |


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
