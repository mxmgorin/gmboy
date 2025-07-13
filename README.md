## 🎮 gmboy

`gmboy` is a Game Boy emulator built in Rust as a deep dive into how emulation and hardware works.  
Built for fun and learning, it’s a project about clean code, deep hardware understanding, and striving for high accuracy and test coverage.
It's not just an emulator — it's a playground for experimentation.

## Supports

-  **Full CPU Emulation**  
  Full instruction set of Sharp LR35902 CPU with accurate timing (instruction and sub-instruction).

- **Full PPU Emulation (Graphics)**  
  Rendering of background, window, and sprites.

- **Full APU Emulation (Audio)**  
  Sound output with all 4 audio channels.

- **Input Handling**  
  Emulates button inputs (D-Pad, A, B, Start, Select).

- **Cartridge Support**  
  Handles ROM loading and MBC0, MBC1, MBC2.

## Features

- **Multiple Palettes**  
  Includes different color palettes with ability to add through config file.

- **Slow and Turbo Modes**  
  Ability to slow down and speed up emulation.

- **Rewind**
  Go back in time! Undo your actions and retry sections without restarting.

- **Save states**
  Save your game progress and resume from the exact same point at any time — no need to rely on in-game save systems.

- **Tests**  
  Integrated SM83 json tests, blargg tests, mooneye test suite (throught `cargo test`).

## Test Results

- ### SM83: 
Passes all of 356000 tests successfully ✅

- ### Blargg

| CPU Instructions          | Memory Timing         | OAM Bug               |
|---------------------------|-----------------------|-----------------------|
| 01-special.gb ✅           | 01-read_timing.gb ✅   | 3-non_causes.gb ✅  |
| 02-interrupts.gb ✅        | 02-write_timing.gb ✅  | 6-timing_no_bug.gb ✅|
| 03-op sp,hl.gb ✅          | 03-modify_timing.gb ✅ | 7-timing_effect.gb ✅|
| 04-op r,imm.gb ✅          |                       | 8-instr_effect.gb ✅|
| 05-op rp.gb ✅             |                       |                       |
| 06-ld r,r.gb ✅            |                       |                       |
| 07-jr,jp,call,ret,rst ✅   |                       |                       |
| 08-misc instrs.gb ✅       |                       |                       |
| 09-op r,r.gb ✅            |                       |                       |
| 10-bit ops.gb ✅           |                       |                       |
| 11-op a,(hl).gb ✅         |                       |                       |
| instr_timing.gb ❌         |                       |                       |
| instr_timing.gb ❌         |                       |                       |

- ### Mooneye

| General & OAM DMA            | Timing                       | Timer Accuracy                 |
|------------------------------|------------------------------|-------------------------------|
| add_sp_e_timing.gb ✅         | call_cc_timing.gb ✅          | div_write.gb ✅             |
| bits/mem_oam.gb ✅            | call_cc_timing2.gb ✅         | rapid_toggle.gb ✅          |
| bits/reg_f.gb ✅              | call_timing.gb ✅             | tim00.gb ✅                 |
| instr/daa.gb ✅               | call_timing2.gb ✅            | tim00_div_trigger.gb ✅     |
| oam_dma/basic.gb ✅           | div_timing.gb ✅              | tim01.gb ✅                 |
| oam_dma/reg_read.gb ✅        | ei_timing.gb ✅               | tim01_div_trigger.gb ✅     |
| oam_dma/oam_dma_restart.gb ✅ | halt_ime0_ei.gb ✅            | tim10.gb ✅                 |
| oam_dma/oam_dma_start.gb ✅   | halt_ime0_nointr_timing.gb ✅ | tim10_div_trigger.gb ✅     |
| oam_dma/oam_dma_timing.gb ✅  | halt_ime1_timing.gb ✅        | tim11.gb ✅                 |
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
|                              | di_timing-GS.gb ❌            |                               |
|                              | intr_timing.gb ❌             |                               |

## ⌨️ Keybindings

- ### Controls

| Key               | Action      |
|-------------------|-------------|
| Arrow Up          | Dpad Up     |
| Arrow Down        | Dpad Down   |
| Arrow Left        | Dpad Left   |
| Arrow Right       | Dpad Right  |
| Z                 | B           |
| X                 | A           |
| Enter (Return)    | Start       |
| Backspace         | Select      |

- ### Emulator Functions

| Action                      | Keyboard                            | Gamepad        |
|-----------------------------|-------------------------------------|----------------|
| Toggle **Rewind** (hold)    | Left Ctrl / Right Ctrl              | Y              |
| Toggle **Turbo** (hold)     | Tab                                 | Right Shoulder |
| Toggle **Slow motion** (hold)| Left Shift / Right Shift           | Left Shoulder  |
| Pause                       | Space                               |                |
| Restart                     | R                                   |                |
| Increase screen scale       | + (Equals)                          |                |
| Decrease screen scale       | - (Minus)                           |                |
| Toggle fullscreen           | F                                   |                |
| Mute audio                  | M                                   |                |
| Cycle color palettes        | P                                   | X              |
| Load save state             | F1–F19                              | Right Trigger  |
| Create save state           | 1–9                                 | Left Trigger   |



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

- [ ] SRAM saving (battery-backed saves)
- [ ] More MBC types (MBC3, MBC5, etc.)
- [ ] Re-mappable keybindings
- [ ] Emulator menu (with settings etc.)
- [ ] Accuracy improvements, bug fixes
- [ ] Shaders
- [ ] GBC mode

## References

Here are some useful resources for Game Boy development and emulation:

- [Game Boy Complete Technical Reference](https://gbdev.io/pandocs/)
- [Game Boy CPU Opcodes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Gbops, an accurate opcode table for the Game Boy](https://izik1.github.io/gbops/index.html)
- [RGBDS GBZ80 Assembly Documentation](https://rgbds.gbdev.io/docs/v0.9.0/gbz80.7)
- [A curated list of Game Boy development resources](https://github.com/gbdev/awesome-gbdev)

## Acknowledgments

- [SM83 Tests](https://github.com/SingleStepTests/sm83)
- [GB Test ROMs](https://github.com/retrio/gb-test-roms)
- [mooneye test suite](https://github.com/Gekkio/mooneye-test-suite)
- [DMG acid2 Test](https://github.com/mattcurrie/dmg-acid2)