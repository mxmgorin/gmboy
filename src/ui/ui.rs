use crate::bus::Bus;
use crate::ppu::{Ppu, LCD_X_RES, LCD_Y_RES};
use crate::ui::debug_window::DebugWindow;
use crate::ui::events::{UiEvent, UiEventHandler};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SCALE: u32 = 4;
pub const SPACER: i32 = (8 * SCALE) as i32;
pub const TILE_ROWS: i32 = 24;
pub const TILE_COLS: i32 = 16;
pub const Y_SPACER: i32 = SCALE as i32;
pub const X_DRAW_START: i32 = (SCALE / 2) as i32;

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    event_pump: EventPump,

    main_canvas: Canvas<Window>,
    debug_window: Option<DebugWindow>,
}

impl Ui {
    pub fn new(debug: bool) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let main_window = video_subsystem
            .window(
                "Main Window",
                LCD_X_RES as u32 * SCALE,
                LCD_Y_RES as u32 * SCALE,
            )
            .position_centered()
            .build()
            .unwrap();
        let main_canvas = main_window.into_canvas().build().unwrap();

        let (x, y) = main_canvas.window().position();
        let mut debug_window = DebugWindow::new(&video_subsystem);
        debug_window.set_position(x + SCREEN_WIDTH as i32 + 10, y);

        Ok(Ui {
            event_pump: sdl_context.event_pump()?,
            _sdl_context: sdl_context,
            //ttf_context,
            main_canvas,
            debug_window: if debug { Some(debug_window) } else { None },
        })
    }

    pub fn draw(&mut self, ppu: &Ppu, bus: &Bus) {
        self.draw_main(ppu);

        if let Some(debug_window) = self.debug_window.as_mut() {
            debug_window.draw(bus);
        }
    }

    fn draw_main(&mut self, ppu: &Ppu) {
        let mut rect = Rect::new(0, 0, SCALE, SCALE);
        self.main_canvas.clear();

        for y in 0..(LCD_Y_RES as usize) {
            for x in 0..(LCD_X_RES as usize) {
                let pixel = ppu.pipeline.buffer[x + (y * LCD_X_RES as usize)];
                rect.x = x as i32 * SCALE as i32;
                rect.y = y as i32 * SCALE as i32;
                let (r, g, b, a) = pixel.color.as_rgba();
                self.main_canvas.set_draw_color(Color::RGBA(r, g, b, a));
                self.main_canvas.fill_rect(rect).unwrap();
            }
        }

        //fill_rects2(&mut self.main_canvas, &self.rects_by_colors, rects_count);
        self.main_canvas.present();
    }

    pub fn handle_events(&mut self, bus: &mut Bus, event_handler: &mut impl UiEventHandler) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::DropFile {filename, ..} => event_handler.on_event(bus, UiEvent::DropFile(filename)),
                Event::Quit { .. } => event_handler.on_event(bus, UiEvent::Quit),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => event_handler.on_event(bus, UiEvent::Key(keycode, true)),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => event_handler.on_event(bus, UiEvent::Key(keycode, false)),
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
}
