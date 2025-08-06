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
        app::run(args);
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
