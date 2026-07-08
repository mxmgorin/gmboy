# GMBoy — Web (WASM) frontend

Runs the `core` emulator in the browser via `wasm-bindgen`. Video + keyboard input
work today; audio is the next step (see repo `docs/ROADMAP.md`).

## Build & run

```bash
cargo install wasm-pack   # once
./build.sh                # builds ./pkg
python3 -m http.server -d . 8080
# open http://localhost:8080/  and load a .gb / .gbc ROM
```

## Controls

| Game Boy | Key         |
| -------- | ----------- |
| D-pad    | Arrow keys  |
| A        | `X`         |
| B        | `Z`         |
| Start    | `Enter`     |
| Select   | `Backspace` |

## Layout

- `src/lib.rs` — the `GmBoy` wasm-bindgen wrapper around `core::emu::Emu`.
- `js/main.js` — page glue: ROM loading, input, and the fixed-timestep run loop.
- `js/audio.js` — `AudioScheduler`, the WebAudio playback queue.
- `index.html` — markup only; loads `js/main.js` as a module.
- `assets/` — bundled demo ROM (see `assets/README.md`).

## How it works

`GmBoy` (in `src/lib.rs`) wraps `core::emu::Emu` and exposes `run_frame()`,
`frame_buffer()` (RGBA), `take_audio()` (interleaved stereo f32), and `set_button()`.

`js/main.js` runs a **fixed-timestep loop**: it steps the emulator at the real
Game Boy frame rate (~59.73 Hz) using an accumulator, independent of the display's
refresh rate, so 144 Hz monitors don't run games at 2.4× speed. Each frame the RGBA
buffer is blitted to a `<canvas>` and the APU samples are handed to `AudioScheduler`,
which schedules them back-to-back on a `<canvas>`-gesture-started `AudioContext`.

We call `emu.runtime.run_frame()` directly (skipping the desktop frontend's internal
sleep/spin timing) because pacing is handled in JS.

Not part of the root Cargo workspace (it's `exclude`d) so a plain desktop
`cargo build` never tries to compile this wasm-only crate for the host.
