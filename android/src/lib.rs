#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_gmboy_MainActivity_nativeMain(_: *const (), _: *const ()) {
    app::run(vec![]);
}

#[no_mangle]
pub extern "C" fn SDL_main(_argc: i32, _argv: *const *const u8) -> i32 {
    log("SDL_main entered");

    app::run(vec![]);
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
        __android_log_print(3, tag.as_ptr() as *const i8, cmsg.as_ptr() as *const i8);
    }
}
