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
  Rewind gameplay to undo mistakes.

-  **Tests**  
   Integrated SM83 json tests, blargg tests, mooneye test suite (throught `cargo test`).

## ⌨️ Keybindings

### Controls

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


### Emulator Functions

| Key                    | Action                          |
|------------------------|--------------------------------|
| Left Ctrl / Right Ctrl | Toggle **Rewind** mode (hold)  |
| Tab                    | Toggle **Turbo** mode (hold)   |
| Left Shift / Right Shift | Toggle **Slow motion** mode (hold) |
| Space                  | Pause                          |
| R                      | Restart                        |
| + (Equals)             | Increase scale                 |
| - (Minus)              | Decrease scale                 |
| F                      | Toggle fullscreen              |
| M                      | Mute audio                    |
| P                      | Cycle through color palettes  |

## 🛠️ Building

First, make sure you have Rust installed. If you don't, install it with:
````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````
Then, install dependencies: SDL2

Arch Linux:
```bash
sudo pacman -S sdl2
````

After that, you should be able to build:
```bash
cargo build --release
```
## Planned features

- [ ] Save states
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