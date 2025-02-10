use crate::bus::Bus;
use crate::emu::EventHandler;
use crate::ppu::tile::{TILE_BITS_COUNT, TILE_BYTE_SIZE, TILE_COLS, TILE_HEIGHT, TILE_ROWS};
use crate::ppu::vram::VRAM_ADDR_START;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowPos};
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const SCALE: u32 = 4;

const TILE_SDL_COLORS: [Color; 4] = [
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
}

impl Ui {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        // let ttf_context = sdl2::ttf::init().unwrap();
        let video_subsystem = sdl_context.video()?;
        let width = 16 * 8 * SCALE;
        let height = 32 * 8 * SCALE;

        let main_window = video_subsystem
            .window("Main Window", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let main_canvas = main_window.into_canvas().build().unwrap();

        let debug_window = video_subsystem
            .window("Debug Window", width, height)
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
        })
    }

    pub fn draw_tile(canvas: &mut Canvas<Window>, bus: &Bus, tile_addr: u16, x: i32, y: i32) {
        let mut rect = Rect::new(0, 0, SCALE, SCALE);

        for tile_y in (0..TILE_HEIGHT).step_by(2) {
            let tile_addr = tile_addr + tile_y as u16;
            let tile = bus.ppu.video_ram.get_tile_pixel(tile_addr);

            for bit in 0..TILE_BITS_COUNT {
                rect.set_x(x + bit * SCALE as i32);
                rect.set_y(y + (tile_y as i32 / 2 * SCALE as i32));
                canvas.set_draw_color(TILE_SDL_COLORS[tile.get_color_index(bit)]);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }

    pub fn draw_debug(&mut self, bus: &Bus) {
        const SPACER: i32 = (8 * SCALE) as i32;
        const Y_SPACER: i32 = SCALE as i32;
        const X_DRAW_START: i32 = (SCALE / 2) as i32;

        let mut x_draw = X_DRAW_START;
        let mut y_draw: i32 = 0;
        let mut tile_num = 0;

        self.debug_canvas.set_draw_color(Color::RGB(21, 21, 21));
        self.debug_canvas
            .fill_rect(None)
            .unwrap();

        for y in 0..TILE_ROWS {
            for x in 0..TILE_COLS {
                let row_start_addr = VRAM_ADDR_START + (tile_num * TILE_BYTE_SIZE);
                Ui::draw_tile(
                    &mut self.debug_canvas,
                    bus,
                    row_start_addr,
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

    pub fn draw(&mut self, bus: &Bus) {
        self.draw_main();
        self.draw_debug(bus);
    }

    pub fn draw_main(&mut self) {
        self.main_canvas.clear();
        self.main_canvas.present();
    }

    pub fn handle_events(&mut self, event_handler: &mut impl EventHandler) {
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
                } => event_handler.on_quit(),
                _ => {}
            }
        }
    }
}

pub fn into_sdl_color(color: u32) -> Color {
    Color::RGBA(
        ((color >> 24) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    )
}
