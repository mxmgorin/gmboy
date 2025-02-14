use crate::bus::Bus;
use crate::ppu::tile::TileData;
use crate::ppu::{Ppu, LCD_X_RES, LCD_Y_RES};
use crate::ui::debug_window::DebugWindow;
use crate::ui::events::{SdlEventHandler, UiEvent, UiEventHandler};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SCALE: u32 = 4;
pub const SPACER: i32 = (8 * SCALE) as i32;
pub const TILE_ROWS: i32 = 24;
pub const TILE_COLS: i32 = 16;
pub const Y_SPACER: i32 = SCALE as i32;
pub const X_DRAW_START: i32 = (SCALE / 2) as i32;

const SDL_COLORS: [Color; 4] = [
    Color::WHITE,
    Color::RGB(170, 170, 170), // Light Gray
    Color::RGB(85, 85, 85),    // Dark Gray
    Color::BLACK,
];

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    main_canvas: Canvas<Window>,
    event_handler: SdlEventHandler,
    // pre-allocated for use in draw function
    frame_rects: [Vec<Rect>; 4],
    debug_window: Option<DebugWindow>,
}

impl Ui {
    pub fn new(debug: bool) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let main_window = video_subsystem
            .window("Main Window", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let main_canvas = main_window.into_canvas().build().unwrap();

        let (x, y) = main_canvas.window().position();
        let mut debug_window = DebugWindow::new(&video_subsystem);
        debug_window.set_position(x + SCREEN_WIDTH as i32 + 10, y);

        Ok(Ui {
            event_handler: SdlEventHandler::new(&sdl_context),
            _sdl_context: sdl_context,
            //ttf_context,
            main_canvas,
            frame_rects: allocate_rects_group(LCD_Y_RES as usize * LCD_X_RES as usize),
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
        let mut rects_count: [usize; 4] = [0; 4];

        for y in 0..(LCD_Y_RES as usize) {
            for x in 0..(LCD_X_RES as usize) {
                let pixel = ppu.pipeline.buffer[x + (y * LCD_X_RES as usize)];
                let color_index = pixel.color_id as usize;
                let rect = &mut self.frame_rects[color_index][rects_count[color_index]];
                rect.x = x as i32 * SCALE as i32;
                rect.y = y as i32 * SCALE as i32;
                rects_count[color_index] += 1;
            }
        }

        self.main_canvas.clear();
        fill_rects(&mut self.main_canvas, &self.frame_rects, rects_count);
        self.main_canvas.present();
    }

    pub fn handle_events(&mut self, event_handler: &mut impl UiEventHandler) {
        if let Some(window_id) = self.event_handler.handle(event_handler) {
            if let Some(window) = self.debug_window.as_mut() {
                if window.canvas.window().id() == window_id {
                    self.debug_window = None;
                } else {
                    event_handler.on_event(UiEvent::Quit);
                }
            }
        }
    }
}

pub fn allocate_rects_group(len: usize) -> [Vec<Rect>; 4] {
    let mut recs = Vec::with_capacity(len);
    for _ in 0..recs.capacity() {
        recs.push(Rect::new(0, 0, SCALE, SCALE));
    }

    [recs.clone(), recs.clone(), recs.clone(), recs]
}

pub fn set_tile_recs(recs: &mut [Vec<Rect>; 4], tile: TileData, x: i32, y: i32) -> [usize; 4] {
    let mut rects_count: [usize; 4] = [0; 4];

    for (line_y, lines) in tile.lines.iter().enumerate() {
        for (bit, color_id) in lines.iter_color_ids().enumerate() {
            let rect = &mut recs[color_id as usize][rects_count[color_id as usize]];
            rect.x = x + (bit as i32 * SCALE as i32);
            rect.y = y + (line_y as i32 * SCALE as i32);
            rects_count[color_id as usize] += 1;
        }
    }

    rects_count
}

pub fn fill_rects(canvas: &mut Canvas<Window>, recs: &[Vec<Rect>; 4], rects_count: [usize; 4]) {
    for (color_id, rects) in recs.iter().enumerate() {
        canvas.set_draw_color(SDL_COLORS[color_id]);
        canvas.fill_rects(&rects[..rects_count[color_id]]).unwrap();
    }
}
