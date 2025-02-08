use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub trait EventHandler {
    fn on_quit(&mut self);
}

pub struct Ui {
    window_canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Ui {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let sdl_window = video_subsystem
            .window("Game Boy", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut window_canvas = sdl_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        window_canvas.set_draw_color(Color::RGB(0, 0, 0));
        window_canvas.clear();
        window_canvas.present();

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            window_canvas,
            event_pump,
        })
    }

    pub fn update(&mut self) {
        self.window_canvas.clear();
        self.window_canvas.present();
    }

    pub fn handle_events(&mut self, event_handler: &mut impl EventHandler) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    event_handler.on_quit();
                }
                _ => {}
            }
        }
    }
}
