use crate::config::DesktopEmuConfig;
use crate::ui::audio::GameAudio;
use crate::ui::debug_window::DebugWindow;
use crate::ui::events::UiEvent;
use crate::ui::text::*;
use crate::Emu;
use core::emu::EmuCallback;
use core::into_pallet;
use core::ppu::tile::{Pixel, PixelColor, TileData};
use core::ppu::{LCD_X_RES, LCD_Y_RES};
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::{EventPump, GameControllerSubsystem};

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const BYTES_PER_PIXEL: usize = 4;

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    game_controller_subsystem: GameControllerSubsystem,
    event_pump: EventPump,

    main_canvas: Canvas<Window>,
    texture: Texture,
    overlay_texture: Texture,
    fps_texture: Texture,
    debug_window: Option<DebugWindow>,
    layout: Layout,
    game_controllers: Vec<GameController>,

    audio: GameAudio,
    pub curr_palette: [PixelColor; 4],
    pub config: DesktopEmuConfig,
}

impl EmuCallback for Ui {
    fn update_video(&mut self, buffer: &[Pixel], fps: usize) {
        self.draw_main(buffer, fps, self.curr_palette[3]);
    }

    fn update_audio(&mut self, output: &[f32]) {
        self.audio.update(output);
    }
}

pub struct Layout {
    pub spacer: i32,
    pub y_spacer: i32,
    pub x_draw_start: i32,
    pub win_width: u32,
    pub win_height: u32,
}

impl Layout {
    pub fn new(scale: f32) -> Self {
        Self {
            spacer: 8 * scale as i32,
            y_spacer: scale as i32,
            x_draw_start: scale as i32 / 2,
            win_width: LCD_X_RES as u32 * scale as u32,
            win_height: LCD_Y_RES as u32 * scale as u32,
        }
    }
}

impl Ui {
    pub fn new(config: DesktopEmuConfig, debug: bool) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let layout = Layout::new(config.graphics.scale);

        let main_window = video_subsystem
            .window("GMBoy", layout.win_width, layout.win_height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut main_canvas = main_window.into_canvas().build().unwrap();
        let texture_creator = main_canvas.texture_creator();
        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, LCD_X_RES as u32, LCD_Y_RES as u32)
            .unwrap();

        if config.graphics.is_fullscreen {
            main_canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
        }

        let debug_window = if debug {
            let (x, y) = main_canvas.window().position();
            let mut debug_window = DebugWindow::new(&video_subsystem);
            debug_window.set_position(x + SCREEN_WIDTH as i32 + 10, y);

            Some(debug_window)
        } else {
            None
        };

        let mut overlay_texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, LCD_X_RES as u32, LCD_Y_RES as u32)
            .unwrap();
        overlay_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        let mut fps_texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, 50, 50)
            .unwrap();
        fps_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut game_controllers = vec![];
        let game_controller_subsystem = sdl_context.game_controller()?;

        for id in 0..game_controller_subsystem.num_joysticks()? {
            if game_controller_subsystem.is_game_controller(id) {
                let controller = game_controller_subsystem.open(id).unwrap();
                game_controllers.push(controller);
            }
        }
        let scale = config.graphics.scale;

        let mut ui = Ui {
            event_pump: sdl_context.event_pump()?,
            game_controller_subsystem,
            main_canvas,
            debug_window,
            layout,
            curr_palette: into_pallet(
                &config.graphics.pallets[config.graphics.selected_pallet_idx].hex_colors,
            ),
            texture,
            overlay_texture,
            fps_texture,
            audio: GameAudio::new(&sdl_context),
            game_controllers,

            _sdl_context: sdl_context,
            config,
        };

        ui.set_scale(scale)?;

        Ok(ui)
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<(), String> {
        self.layout = Layout::new(scale);
        let window = self.main_canvas.window_mut();
        window
            .set_size(self.layout.win_width, self.layout.win_height)
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );

        Ok(())
    }

    pub fn draw_debug(&mut self, tiles: impl Iterator<Item = TileData>) {
        if let Some(debug_window) = self.debug_window.as_mut() {
            debug_window.draw_tiles(tiles);
        }
    }

    pub fn draw_text(&mut self, text: &str) {
        self.main_canvas.clear();

        let (win_width, win_height) = self.main_canvas.window().size();
        let text_width = calc_text_width(text, self.config.graphics.text_scale);
        // Calculate the x and y positions to center the text
        let x = (LCD_X_RES as u32 as usize - text_width) / 2;
        let y = (LCD_Y_RES as u32 as usize - get_text_height(self.config.graphics.text_scale)) / 2;

        fill_texture(&mut self.overlay_texture, self.curr_palette[3]);

        draw_text(
            &mut self.overlay_texture,
            text,
            self.curr_palette[0],
            x,
            y,
            self.config.graphics.text_scale,
        );
        let dest_rect = calculate_scaled_rect(win_width, win_height);

        self.main_canvas
            .copy(&self.overlay_texture, None, Some(dest_rect))
            .unwrap();

        self.main_canvas.present();
    }

    pub fn draw_main(&mut self, pixel_buffer: &[Pixel], fps: usize, text_color: PixelColor) {
        self.main_canvas.clear();

        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..LCD_Y_RES as usize {
                    for x in 0..LCD_X_RES as usize {
                        let pixel = pixel_buffer[x + (y * LCD_X_RES as usize)];
                        let (r, g, b, a) = pixel.color.as_rgba();
                        let offset = (y * pitch) + (x * BYTES_PER_PIXEL);
                        buffer[offset] = r;
                        buffer[offset + 1] = g;
                        buffer[offset + 2] = b;
                        buffer[offset + 3] = a;
                    }
                }
            })
            .unwrap();

        let (win_width, win_height) = self.main_canvas.window().size();
        let dest_rect = calculate_scaled_rect(win_width, win_height);

        // Copy the texture while maintaining aspect ratio
        self.main_canvas
            .copy(&self.texture, None, Some(dest_rect))
            .unwrap();

        if self.config.graphics.show_fps {
            let text = fps.to_string();
            fill_texture(&mut self.fps_texture, PixelColor::from_hex(0));
            draw_text(&mut self.fps_texture, &text, text_color, 5, 5, 1);

            self.main_canvas
                .copy(&self.fps_texture, None, Some(Rect::new(0, 0, 80, 80)))
                .unwrap();
        }

        self.main_canvas.present();
    }

    /// Polls and handles events. Returns false on quit.
    pub fn handle_events(&mut self, emu: &mut Emu) -> bool {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(controller) = self.game_controller_subsystem.open(which) {
                        self.game_controllers.push(controller);
                        println!("Controller {which} connected");
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    self.game_controllers.retain(|c| c.instance_id() != which);
                    println!("Controller {which} disconnected");
                }
                Event::DropFile { filename, .. } => {
                    self.on_event(emu, UiEvent::FileDropped(filename.into()))
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(emu, keycode, true) {
                        self.on_event(emu, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(emu, keycode, false) {
                        self.on_event(emu, evt);
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(emu, button, true) {
                        self.on_event(emu, evt);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(evt) = self.handle_controller_button(emu, button, false) {
                        self.on_event(emu, evt);
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    if let Some(evt) = self.handle_joy_axis(axis_idx, value) {
                        self.on_event(emu, evt);
                    }
                }
                Event::MouseButtonDown { .. } => {
                    self.on_event(emu, UiEvent::PickFile);
                }
                Event::Quit { .. } => return false,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = self.debug_window.as_mut() {
                        if window.canvas.window().id() == window_id {
                            self.debug_window = None;
                        } else {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn next_palette(&mut self, emu: &mut Emu) {
        self.config.graphics.selected_pallet_idx = get_next_pallet_idx(
            self.config.graphics.selected_pallet_idx,
            self.config.graphics.pallets.len() - 1,
        );
        let pallet = &self.config.graphics.pallets[self.config.graphics.selected_pallet_idx];
        self.curr_palette = into_pallet(
            &pallet.hex_colors,
        );
        emu.cpu.bus.io.lcd.set_pallet(self.curr_palette);
        emu.ctx.config.pallet = pallet.hex_colors.clone();
        self.sync_settings(emu);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.config.graphics.is_fullscreen = !self.config.graphics.is_fullscreen;

        if self.config.graphics.is_fullscreen {
            self.main_canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                .unwrap();
        } else {
            self.main_canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Off)
                .unwrap();
        }
    }

    pub fn sync_settings(&mut self, emu: &mut Emu) {
        self.config.emulation = emu.ctx.config.clone();
    }
}

fn calculate_scaled_rect(window_width: u32, window_height: u32) -> Rect {
    let screen_aspect = window_width as f32 / window_height as f32;
    let game_aspect = LCD_X_RES as f32 / LCD_Y_RES as f32;

    let (new_width, new_height) = if screen_aspect > game_aspect {
        // Screen is wider than game: Fit height, adjust width
        let height = window_height;
        let width = ((height as f32) * game_aspect) as u32;
        (width, height)
    } else {
        // Screen is taller than game: Fit width, adjust height
        let width = window_width;
        let height = ((width as f32) / game_aspect) as u32;
        (width, height)
    };

    // Center the image in the screen
    let x = ((window_width - new_width) / 2) as i32;
    let y = ((window_height - new_height) / 2) as i32;

    Rect::new(x, y, new_width, new_height)
}

pub fn get_next_pallet_idx(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}
