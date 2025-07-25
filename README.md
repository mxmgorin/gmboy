## GMBoy

`GMBoy` is a Game Boy emulator built in Rust as a deep dive into how emulation and hardware works. Built for fun and learning, it emphasizes clean code, hardware understanding, and aims for high accuracy, test coverage, and good performance. 

The emulator implements most Game Boy functionality and accuracy is verified by community test suites like blargg's and [mooneye](https://github.com/Gekkio/mooneye-test-suite). While most games should run correctly, some issues may still occur.

Download the [latest release](https://github.com/mxmgorin/gmboy/releases/latest) of `GMBoy` from the releases page.

## Features

- **Config Management**  
  Customize the config.json file via a built-in menu or by editing it manually.

- **Multiple Palettes**  
  Includes different color palettes with ability to add by editing the palettes.json file.

- **Slow and Turbo Modes**  
  Ability to slow down or speed up emulation. Speed is adjustable via the config.

- **Rewind**  
  Undo your actions and retry sections without restarting. Adjustable via the config.

- **Save States**  
  Save your game progress and resume where you left off. Auto save states on exit/start can be toggled via the config.

- **Tests**  
  Integrated SM83 json tests, blargg tests, mooneye test suite (via `cargo test`).

- **Tile Viewer**  
  Visualize and inspect background and sprite tile data in real time. Toggle it via the config.

## Keybindings

| Action                  | ⌨️ Keyboard              | 🎮 Gamepad        |
|-------------------------|--------------------------|-------------------|
| D-pad Up                | Arrow Up                 | D-pad Up          |
| D-pad Down              | Arrow Down               | D-pad Down        |
| D-pad Left              | Arrow Left               | D-pad Left        |
| D-pad Right             | Arrow Right              | D-pad Right       |
| B                       | Z                        | B                 |
| A                       | X                        | A                 |
| Start                   | Enter (Return)           | Start             |
| Select                  | Backspace                | Select            |
| Rewind (hold)           | Left Ctrl / Right Ctrl   | Y                 |
| Turbo mode (hold)       | Tab                      | Right Shoulder    |
| Slow mode (hold)        | Left Shift / Right Shift | Left Shoulder     |
| Pause                   | Space                    |                   |
| Restart                 | R                        |                   |
| Screen scale up         | + (Equals)               |                   |
| Screen scale down       | - (Minus)                |                   |
| Fullscreen Toggle       | F                        |                   |
| Mute audio              | M                        |                   |
| Change color palette    | P                        | X                 |
| Load save state (1–9)   | F1–F19                   | Right Trigger (1) |
| Create save state (1–9) | 1–9                      | Left Trigger (1)  |
| Volume up               | F12                      |                   |
| Volume down             | F11                      |                   |

## Supports

-  **Full CPU Emulation**  
   Complete implementation of the Sharp LR35902 instruction set with accurate instruction and sub-instruction timing.

- **Full PPU Emulation (Graphics)**  
  Rendering of background, window, and sprites.

- **Full APU Emulation (Audio)**  
  Sound output with all 4 audio channels (Square 1, Square 2, Wave, Noise).

- **Cartridge MBCs**  
  Supports ROM loading with No MBC, MBC1, MBC1M, MBC2, MBC3, MBC5 mappers.

- **SRAM Battery Saves**  
  Persistent save data using battery-backed SRAM.

- **Input Handling**  
  Emulates all standard Game Boy button inputs (D-Pad, A, B, Start, Select).

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
## Planned features

- GBC mode
- Re-mappable keybindings
- Accuracy improvements, bug fixes
- Shaders
- Web, android, arm builds
- Audio visualizer
- Palette loader

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
| unused_hwio-GS.gb ❌          | halt_ime1_timing2-GS.gb ✅    | tim11_div_trigger.gb ✅     |
| ie_push.gb ❌                 | jp_cc_timing.gb ✅            | tima_reload.gb ✅           |
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
|                              | di_timing-GS.gb ❌            |                               |
|                              | intr_timing ❌                |                               |

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


## References

Here are some useful resources for Game Boy development and emulation:

- [Game Boy Complete Technical Reference](https://gbdev.io/pandocs/)
- [Gekkio's Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
- [Game Boy CPU Opcodes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Gbops, an accurate opcode table for the Game Boy](https://izik1.github.io/gbops/index.html)
- [RGBDS GBZ80 Assembly Documentation](https://rgbds.gbdev.io/docs/v0.9.0/gbz80.7)
- [A curated list of Game Boy development resources](https://github.com/gbdev/awesome-gbdev)

## Acknowledgments

- [SM83 Tests](https://github.com/SingleStepTests/sm83)
- [GB Test ROMs](https://github.com/retrio/gb-test-roms)
- [mooneye test suite](https://github.com/Gekkio/mooneye-test-suite)
- [DMG acid2 Test](https://github.com/mattcurrie/dmg-acid2)