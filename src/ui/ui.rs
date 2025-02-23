use crate::bus::Bus;
use crate::config::GraphicsConfig;
use crate::ppu::{Ppu, LCD_X_RES, LCD_Y_RES};
use crate::tile::PixelColor;
use crate::ui::debug_window::DebugWindow;
use crate::ui::events::{UiEvent, UiEventHandler};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    event_pump: EventPump,

    canvas: Canvas<Window>,
    texture: Texture,
    debug_window: Option<DebugWindow>,
    layout: Layout,

    pub config: GraphicsConfig,
    pub curr_palette: [PixelColor; 4],
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
    pub fn new(config: GraphicsConfig, debug: bool) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let layout = Layout::new(config.scale);

        let main_window = video_subsystem
            .window("GMBoy", layout.win_width, layout.win_height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut main_canvas = main_window.into_canvas().build().unwrap();
        main_canvas.set_scale(config.scale, config.scale)?;
        let texture_creator = main_canvas.texture_creator();
        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, LCD_X_RES as u32, LCD_Y_RES as u32)
            .unwrap();

        let (x, y) = main_canvas.window().position();
        let mut debug_window = DebugWindow::new(&video_subsystem);
        debug_window.set_position(x + SCREEN_WIDTH as i32 + 10, y);

        Ok(Ui {
            event_pump: sdl_context.event_pump()?,
            _sdl_context: sdl_context,
            canvas: main_canvas,
            debug_window: if debug { Some(debug_window) } else { None },
            layout,
            curr_palette: into_pallet(&config.pallets[config.selected_pallet_idx].hex_colors),
            config,
            texture,
        })
    }

    fn set_scale(&mut self, scale: f32) -> Result<(), String> {
        self.config.scale = scale;
        self.layout = Layout::new(scale);
        self.canvas.set_scale(scale, scale)?;
        let window = self.canvas.window_mut();
        window
            .set_size(self.layout.win_width, self.layout.win_height)
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );

        Ok(())
    }

    pub fn draw(&mut self, ppu: &Ppu, bus: &Bus) {
        self.draw_main(ppu);

        if let Some(debug_window) = self.debug_window.as_mut() {
            debug_window.draw(bus);
        }
    }

    fn draw_main(&mut self, ppu: &Ppu) {
        const BYTES_PER_PIXEL: usize = 4;
        self.canvas.clear();

        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..LCD_Y_RES as usize {
                    for x in 0..LCD_X_RES as usize {
                        let pixel = ppu.pipeline.buffer[x + (y * LCD_X_RES as usize)];
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

        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn handle_events(&mut self, bus: &mut Bus, event_handler: &mut impl UiEventHandler) {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::DropFile { filename, .. } => {
                    event_handler.on_event(bus, UiEvent::DropFile(filename))
                }
                Event::Quit { .. } => event_handler.on_event(bus, UiEvent::Quit),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(bus, keycode, true) {
                        event_handler.on_event(bus, evt);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(evt) = self.handle_key(bus, keycode, false) {
                        event_handler.on_event(bus, evt);
                    }
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    if let Some(window) = self.debug_window.as_mut() {
                        if window.canvas.window().id() == window_id {
                            self.debug_window = None;
                        } else {
                            event_handler.on_event(bus, UiEvent::Quit);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, bus: &mut Bus, keycode: Keycode, is_down: bool) -> Option<UiEvent> {
        match keycode {
            Keycode::UP => bus.io.joypad.up = is_down,
            Keycode::DOWN => bus.io.joypad.down = is_down,
            Keycode::LEFT => bus.io.joypad.left = is_down,
            Keycode::RIGHT => bus.io.joypad.right = is_down,
            Keycode::Z => bus.io.joypad.b = is_down,
            Keycode::X => bus.io.joypad.a = is_down,
            Keycode::Return => bus.io.joypad.start = is_down,
            Keycode::BACKSPACE => bus.io.joypad.select = is_down,
            Keycode::SPACE => {
                if !is_down {
                    return Some(UiEvent::Pause);
                }
            }
            Keycode::R => {
                if !is_down {
                    return Some(UiEvent::Restart);
                }
            }
            Keycode::EQUALS => {
                if !is_down {
                    self.set_scale(self.config.scale + 1.0).unwrap();
                    return Some(UiEvent::ConfigChanged(self.config.clone()));
                }
            }
            Keycode::MINUS => {
                if !is_down {
                    self.set_scale(self.config.scale - 1.0).unwrap();
                    return Some(UiEvent::ConfigChanged(self.config.clone()));
                }
            }
            Keycode::F => {
                if !is_down {
                    self.toggle_fullscreen();
                    return Some(UiEvent::ConfigChanged(self.config.clone()));
                }
            }
            Keycode::P => {
                if !is_down {
                    self.config.selected_pallet_idx = get_next_pallet_idx(
                        self.config.selected_pallet_idx,
                        self.config.pallets.len() - 1,
                    );
                    self.curr_palette = into_pallet(
                        &self.config.pallets[self.config.selected_pallet_idx].hex_colors,
                    );
                    bus.io.lcd.set_pallet(self.curr_palette);
                    return Some(UiEvent::ConfigChanged(self.config.clone()));
                }
            }
            _ => (), // Ignore other keycodes
        }

        None
    }

    fn toggle_fullscreen(&mut self) {
        self.config.is_fullscreen = !self.config.is_fullscreen;

        if self.config.is_fullscreen {
            self.canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                .unwrap();
        } else {
            self.canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Off)
                .unwrap();
        }
    }
}

pub fn into_pallet(hex_colors: &[String]) -> [PixelColor; 4] {
    let colors: Vec<PixelColor> = hex_colors
        .iter()
        .map(|hex| PixelColor::from_hex(u32::from_str_radix(hex, 16).unwrap()))
        .collect();

    colors[..4].try_into().unwrap()
}

pub fn get_next_pallet_idx(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}
