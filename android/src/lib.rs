use app::app::App;
use app::config::AppConfig;
use app::input::handler::InputHandler;
use app::{load_cart, new_emu};
use core::ppu::palette::LcdPalette;
use std::backtrace::Backtrace;

#[no_mangle]
pub extern "C" fn SDL_main(_argc: i32, _argv: *const *const u8) -> i32 {
    log("SDL_main entered");

    std::panic::set_hook(Box::new(|info| {
        let bt = Backtrace::capture();
        log(&format!("Rust panic: {info}\nBacktrace:\n{bt:?}"));
    }));

    _ = std::panic::catch_unwind(|| {
        let args: Vec<String> = std::env::args().collect();
        let config = AppConfig::default();
        let palettes = LcdPalette::default_palettes().into_boxed_slice();
        let mut emu = new_emu(&config, &palettes);
        let mut sdl = sdl2::init().unwrap();
        let mut input = InputHandler::new(&sdl).unwrap();
        let mut app = App::new(&mut sdl, config, palettes).unwrap();
        load_cart(&mut app, &mut emu, args);

        if let Err(err) = app.run(&mut emu, &mut input) {
            eprintln!("Failed app run: {err}");
        }

        if let Err(err) = app.save_files(&mut emu) {
            eprintln!("Failed app.save_files: {err}");
        }
    });

    0
}

#[link(name = "log")]
extern "C" {
    fn __android_log_print(prio: i32, tag: *const i8, fmt: *const i8, ...) -> i32;
}

pub fn log(msg: &str) {
    use std::ffi::CString;
    let tag = CString::new("gmboy").unwrap();
    let cmsg = CString::new(msg).unwrap();
    unsafe {
        __android_log_print(3, tag.as_ptr() as *const _, cmsg.as_ptr() as *const _);
    }
}
