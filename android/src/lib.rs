mod filesystem;
mod java;
mod native;

use crate::filesystem::AndroidFilesystem;
use jni::objects::JObject;
use jni::{JNIEnv, JavaVM};
use std::backtrace::Backtrace;
use std::sync::OnceLock;

static JVM: OnceLock<JavaVM> = OnceLock::new();

#[no_mangle]
pub extern "C" fn SDL_main(_argc: i32, _argv: *const *const u8) -> i32 {
    log("SDL_main entered");

    std::panic::set_hook(Box::new(|info| {
        let bt = Backtrace::capture();
        log(&format!("Rust panic: {info}\nBacktrace:\n{bt:?}"));
    }));

    _ = std::panic::catch_unwind(|| {
        let args: Vec<String> = std::env::args().collect();
        let file_dialog = AndroidFilesystem;
        app::run(args, Box::new(file_dialog));
    });

    0
}

#[link(name = "log")]
extern "C" {
    fn __android_log_print(prio: i32, tag: *const i8, fmt: *const i8, ...) -> i32;
}

extern "C" {
    fn SDL_AndroidGetActivity() -> *mut std::os::raw::c_void;
    fn SDL_AndroidGetJNIEnv() -> *mut std::os::raw::c_void;
}

pub fn log(msg: &str) {
    use std::ffi::CString;
    let tag = CString::new("gmboy").unwrap();
    let cmsg = CString::new(msg).unwrap();
    unsafe {
        __android_log_print(3, tag.as_ptr() as *const _, cmsg.as_ptr() as *const _);
    }
}

fn get_activity<'a>() -> JObject<'a> {
    unsafe { JObject::from_raw(SDL_AndroidGetActivity() as jni::sys::jobject) }
}

fn get_env() -> JNIEnv<'static> {
    unsafe { JNIEnv::from_raw(SDL_AndroidGetJNIEnv() as *mut _).unwrap() }
}