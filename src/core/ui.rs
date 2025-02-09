use crate::emu::EventHandler;
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

static TILE_COLORS: [u32; 4] = [
    0xFFFFFFFF, // White
    0xFFAAAAAA, // Light Gray
    0xFF555555, // Dark Gray
    0xFF000000, // Black
];

pub struct Ui {
    _sdl_context: sdl2::Sdl,
    //ttf_context: sdl2::ttf::Sdl2TtfContext,
    main_canvas: Canvas<Window>,
    event_pump: EventPump,

    debug_canvas: Canvas<Window>,
    //debug_texture: Texture,
    //debug_surface: Surface<'static>,
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
        let debug_surface = Surface::new(
            width + 16 * SCALE,
            height + 64 * SCALE,
            PixelFormatEnum::ARGB8888,
        )?;
        let debug_texture = debug_canvas
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                width + 16 * SCALE,
                height + 64 * SCALE,
            )
            .map_err(|e| e.to_string())?;
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
            //debug_texture,
            //debug_surface,
        })
    }

    pub fn display_tile(&mut self, addr: u16, tile_num: u16, x: i32, y: i32) {
        let mut rect = Rect::new(0, 0, SCALE, SCALE);

        for tile_y in (0..16).step_by(2) {
            let b1 = bus_read(addr + tile_num * 16 + tile_y);
            let b2 = bus_read(addr + tile_num * 16 + tile_y + 1);

            for bit in (0..8).rev() {
                let hi = ((b1 >> bit) & 1) << 1;
                let lo = (b2 >> bit) & 1;

                let color = TILE_COLORS[(hi | lo) as usize];
                rect.set_x(x + (7 - bit) * SCALE as i32);
                rect.set_y(y + tile_y as i32 / 2 * SCALE as i32);
                self.debug_canvas.fill_rect(rect).unwrap();
                //self.debug_surface
                //    .fill_rect(rect, into_sdl_color(color))
                //    .unwrap();
            }
        }
    }

    pub fn update_debug(&mut self) {
        self.debug_canvas.clear();

        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_num = 0;
        //self.debug_surface
        //    .fill_rect(None, into_sdl_color(0xFF111111))
        //    .unwrap();

        let addr: u16 = 0x8000;

        for _y in 0..24 {
            for _x in 0..16 {
                self.display_tile(addr, tile_num, x_draw, y_draw);
                x_draw += (8 * SCALE) as i32;
                tile_num += 1;
            }
            y_draw += (8 * SCALE) as i32;
            x_draw = 0;
        }

/*        self.debug_texture
            .update(
                None,
                self.debug_surface.without_lock().unwrap(),
                self.debug_surface.pitch() as usize,
            )
            .unwrap();

        self.debug_canvas
            .copy(&self.debug_texture, None, None)
            .unwrap();*/
        self.debug_canvas.present();
    }

    pub fn update(&mut self) {
        self.update_main();
        self.update_debug();
    }
    
    pub fn update_main(&mut self) {
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

fn bus_read(_addr: u16) -> u8 {
    // Dummy implementation for bus_read
    0
}

fn into_sdl_color(color: u32) -> Color {
    Color::RGBA(
        ((color >> 24) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    )
}
