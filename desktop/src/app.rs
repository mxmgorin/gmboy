use crate::audio::AppAudio;
use crate::config::AppConfig;
use crate::input::InputHandler;
use crate::video::game_window::GameWindow;
use crate::video::tiles_window::TileWindow;
use crate::Emu;
use core::emu::battery::BatterySave;
use core::emu::state::RunMode;
use core::emu::state::SaveStateEvent;
use core::emu::EmuCallback;
use core::into_palette;
use core::ppu::tile::PixelColor;
use sdl2::Sdl;
use std::path::{PathBuf};

pub enum AppEvent {
    Pause,
    Rewind,
    FileDropped(PathBuf),
    Restart,
    ModeChanged(RunMode),
    Mute,
    SaveState(SaveStateEvent, usize),
    PickFile,
}

pub struct App {
    audio: AppAudio,
    window: GameWindow,

    pub tiles_window: Option<TileWindow>,
    pub curr_palette: [PixelColor; 4],
    pub config: AppConfig,
}

impl EmuCallback for App {
    fn update_video(&mut self, buffer: &[u32], fps: usize) {
        self.window.draw_buffer(buffer);

        if self.config.interface.show_fps {
            self.window.draw_fps(fps, self.config.interface.text_scale, self.curr_palette[3]);
        }

        self.window.present();
    }

    fn update_audio(&mut self, output: &[f32]) {
        self.audio.queue(output);
    }

    fn paused(&mut self) {
        if self.config.last_cart_path.is_none() {
            self.draw_text(&["NO GAME FILE", "DROP OR PICK IT"], true);
        } else {
            self.draw_text(&["PAUSED"], true);
        }
    }
}

impl App {
    pub fn new(sdl: &mut Sdl, config: AppConfig) -> Result<Self, String> {
        let video_subsystem = sdl.video()?;
        let mut renderer = GameWindow::new(config.interface.scale as u32, &video_subsystem)?;
        renderer.set_fullscreen(config.interface.is_fullscreen);

        let debug_window = if config.interface.tile_viewer {
            let (x, y) = renderer.get_position();
            let mut debug_window = TileWindow::new(&video_subsystem);
            debug_window.set_position(x + 640, y);

            Some(debug_window)
        } else {
            None
        };

        Ok(Self {
            tiles_window: debug_window,
            curr_palette: into_palette(
                &config.interface.palettes[config.interface.selected_palette_idx].hex_colors,
            ),
            audio: AppAudio::new(sdl),
            config,
            window: renderer,
        })
    }

    /// Execution loop
    pub fn run(&mut self, emu: &mut Emu, input: &mut InputHandler) -> Result<(), String> {
        while input.handle_events(self, emu) {
            emu.run_frame(self)?;

            if let Some(tiles_window) = self.tiles_window.as_mut() {
                tiles_window.draw_tiles(emu.runtime.bus.video_ram.iter_tiles());
            }
        }

        Ok(())
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<(), String> {
        self.window.set_scale(scale as u32)?;

        println!("Current scale: {scale}");

        Ok(())
    }

    fn draw_text(&mut self, lines: &[&str], center: bool) {
        let bg_color = self.curr_palette[3];
        let color = self.curr_palette[0];
        self.window.draw_text_lines(
            lines,
            self.config.interface.text_scale,
            color,
            bg_color,
            center,
        );

        self.window.present();
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.interface.selected_palette_idx = get_next_pallet_idx(
            self.config.interface.selected_palette_idx,
            self.config.interface.palettes.len() - 1,
        );
        let pallet = &self.config.interface.palettes[self.config.interface.selected_palette_idx];
        self.curr_palette = into_palette(&pallet.hex_colors);
        emu.runtime.bus.io.lcd.set_pallet(self.curr_palette);

        println!("Current palette: {}", pallet.name);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.interface.is_fullscreen = !self.config.interface.is_fullscreen;
        self.window
            .set_fullscreen(self.config.interface.is_fullscreen);
    }

    pub fn handle_save_state(&self, emu: &mut Emu, event: SaveStateEvent, index: usize) {
        let name = self.config.get_last_file_stem().unwrap();

        match event {
            SaveStateEvent::Create => {
                let save_state = emu.create_save_state();

                if let Err(err) = save_state.save_file(&name, index) {
                    eprintln!("Failed save_state: {err}",);
                }

                println!("Saved save state: {index}");
            }
            SaveStateEvent::Load => {
                let save_state = core::emu::runtime::EmuSaveState::load_file(&name, index);

                let Ok(save_state) = save_state else {
                    eprintln!("Failed load save_state: {}", save_state.unwrap_err());
                    return;
                };

                emu.load_save_state(save_state);
                println!("Loaded save state: {index}");
            }
        }
    }

    pub fn save_files(&mut self, emu: &mut Emu) -> Result<(), String> {
        // save config
        self.config.set_emu(emu.config.clone());
        if let Err(err) = self.config.save_file().map_err(|e| e.to_string()) {
            eprint!("Failed config.save: {err}");
        }

        // save sram for battery emulation
        let name = self.config.get_last_file_stem().unwrap();

        if let Some(bytes) = emu.runtime.bus.cart.dump_ram() {
            let battery = BatterySave::from_bytes(bytes)
                .save_file(&name)
                .map_err(|e| e.to_string());

            if let Err(err) = battery {
                eprint!("Failed BatterySave: {err}");
            };
        }

        // save state on exit
        if self.config.save_state_on_exit {
            if let Err(err) = emu.create_save_state().save_file(&name, 0) {
                eprintln!("Failed save_state: {err}");
            }
        }

        Ok(())
    }
}

fn get_next_pallet_idx(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}
