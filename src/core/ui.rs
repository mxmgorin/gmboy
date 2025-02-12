use crate::bus::Bus;
use crate::ppu::tile::{Tile, TILE_HEIGHT, TILE_TABLE_START, TILE_WIDTH};
use crate::ppu::{Ppu, X_RES, Y_RES};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::{SDL_Rect, SDL_RenderFillRects};
use sdl2::video::{Window, WindowPos};
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const SCALE: u32 = 4;
const SPACER: i32 = (8 * SCALE) as i32;
pub const TILE_ROWS: i32 = 24;
pub const TILE_COLS: i32 = 16;

const SDL_TILE_COLORS: [Color; 4] = [
    Color::WHITE,
    Color::RGB(170, 170, 170), // Light Gray
    Color::RGB(85, 85, 85),    // Dark Gray
    Color::BLACK,
];

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    //ttf_context: sdl2::ttf::Sdl2TtfContext,
    main_canvas: Canvas<Window>,
    event_pump: EventPump,

    debug_canvas: Canvas<Window>,
    tile_rects: [Vec<SDL_Rect>; 4],
}

impl Ui {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        // let ttf_context = sdl2::ttf::init().unwrap();
        let video_subsystem = sdl_context.video()?;
        let tile_grid_width =
            TILE_COLS as u32 * TILE_WIDTH as u32 * SCALE + (TILE_COLS as u32 * SCALE);
        let tile_grid_height = TILE_ROWS as u32 * TILE_HEIGHT as u32 * SCALE + 122;

        let main_window = video_subsystem
            .window("Main Window", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let main_canvas = main_window.into_canvas().build().unwrap();

        let debug_window = video_subsystem
            .window("Debug Window", tile_grid_width, tile_grid_height)
            .position_centered()
            .build()
            .unwrap();
        let mut debug_canvas = debug_window.into_canvas().build().unwrap();

        let (x, y) = main_canvas.window().position();
        debug_canvas.window_mut().set_position(
            WindowPos::Positioned(x + SCREEN_WIDTH as i32 + 10),
            WindowPos::Positioned(y),
        );

        Ok(Ui {
            event_pump: sdl_context.event_pump()?,
            _sdl_context: sdl_context,
            //ttf_context,
            main_canvas,
            debug_canvas,
            tile_rects: allocate_rects_group(TILE_WIDTH as usize * TILE_HEIGHT as usize),
        })
    }

    pub fn draw_tile(&mut self, bus: &Bus, tile_addr: u16, x: i32, y: i32) {
        let tile = bus.video_ram.get_tile(tile_addr);
        let rects_count = fill_tile_recs(&mut self.tile_rects, tile, x, y);

        for (color_id, rects) in self.tile_rects.iter().enumerate() {
            self.debug_canvas.set_draw_color(SDL_TILE_COLORS[color_id]);
            unsafe {
                let result = SDL_RenderFillRects(
                    self.debug_canvas.raw(),
                    rects.as_ptr(),
                    rects_count[color_id] as i32,
                );

                if result != 0 {
                    eprintln!("Error rendering tile: {:?}", result);
                }
            }
        }
    }

    pub fn draw_debug(&mut self, bus: &Bus) {
        const Y_SPACER: i32 = SCALE as i32;
        const X_DRAW_START: i32 = (SCALE / 2) as i32;

        let mut x_draw = X_DRAW_START;
        let mut y_draw: i32 = 0;
        let mut tile_num = 0;

        self.debug_canvas.set_draw_color(Color::RGB(18, 18, 18));
        self.debug_canvas.fill_rect(None).unwrap();

        for y in 0..TILE_ROWS {
            for x in 0..TILE_COLS {
                self.draw_tile(
                    bus,
                    TILE_TABLE_START + (tile_num * TILE_COLS as u16),
                    x_draw + (x * SCALE as i32),
                    y_draw + (y + SCALE as i32),
                );
                x_draw += SPACER;
                tile_num += 1;
            }

            y_draw += SPACER + Y_SPACER;
            x_draw = X_DRAW_START;
        }

        self.debug_canvas.present();
    }

    pub fn draw(&mut self, ppu: &Ppu, bus: &Bus) {
        self.draw_main(ppu);
        self.draw_debug(bus);
    }

    pub fn draw_main(&mut self, ppu: &Ppu) {
        self.main_canvas.clear();
        let mut rect = Rect::new(0, 0, SCALE, SCALE);

        for y in 0..(Y_RES as usize) {
            for x in 0..(X_RES as usize) {
                rect.set_x(x as i32 * SCALE as i32);
                rect.set_y(y as i32 * SCALE as i32);
                let color = ppu.pipeline.buffer[x + (y * X_RES as usize)];
                self.main_canvas.set_draw_color(into_sdl_color(color));
                self.main_canvas.fill_rect(rect).unwrap();
            }
        }

        self.main_canvas.present();
    }

    pub fn handle_events(&mut self, event_handler: &mut impl UiEventHandler) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    ..
                } => event_handler.on_event(UiEvent::Quit),
                _ => {}
            }
        }
    }
}
fn allocate_rects_group(len: usize) -> [Vec<SDL_Rect>; 4] {
    let mut recs = Vec::with_capacity(len);
    for _ in 0..recs.capacity() {
        recs.push(SDL_Rect {
            x: 0,
            y: 0,
            w: SCALE as i32,
            h: SCALE as i32,
        });
    }

    [recs.clone(), recs.clone(), recs.clone(), recs]
}

pub fn fill_tile_recs(recs: &mut [Vec<SDL_Rect>; 4], tile: Tile, x: i32, y: i32) -> [usize; 4] {
    let mut rects_count: [usize; 4] = [0; 4];

    for (line_y, lines) in tile.lines.iter().enumerate() {
        for (bit, color_id) in lines.iter_color_ids().enumerate() {
            let rect = &mut recs[color_id][rects_count[color_id]];
            rect.x = x + (bit as i32 * SCALE as i32);
            rect.y = y + (line_y as i32 * SCALE as i32);
            rects_count[color_id] += 1;
        }
    }

    rects_count
}

pub fn into_sdl_color(color: u32) -> Color {
    Color::RGBA(
        ((color >> 24) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    )
}

pub trait UiEventHandler {
    fn on_event(&mut self, event: UiEvent);
}

pub enum UiEvent {
    Quit,
}
