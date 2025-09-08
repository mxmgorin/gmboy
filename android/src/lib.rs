mod file_dialog;
mod file_system;
mod java;
mod native;

use crate::file_dialog::AndroidFileDialog;
use crate::file_system::AndroidFileSystem;
use android_logger::Config;
use app::AppPlatform;
use jni::objects::JObject;
use jni::{JNIEnv, JavaVM};
use log::LevelFilter;
use std::backtrace::Backtrace;
use std::sync::OnceLock;

static JVM: OnceLock<JavaVM> = OnceLock::new();

#[no_mangle]
pub extern "C" fn SDL_main(_argc: i32, _argv: *const *const u8) -> i32 {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    log::info!("SDL_main entered");

    std::panic::set_hook(Box::new(|info| {
        let bt = Backtrace::capture();
        log::error!("Rust panic: {info}\nBacktrace:\n{bt:?}");
    }));

    _ = std::panic::catch_unwind(|| {
        let args: Vec<String> = std::env::args().collect();
        let fs = AndroidFileSystem;
        let dialog = AndroidFileDialog;
        app::run(args, AppPlatform::new(fs, dialog));
    });

    0
}

extern "C" {
    fn SDL_AndroidGetActivity() -> *mut std::os::raw::c_void;
    fn SDL_AndroidGetJNIEnv() -> *mut std::os::raw::c_void;
}

fn get_activity<'a>() -> JObject<'a> {
    unsafe { JObject::from_raw(SDL_AndroidGetActivity() as jni::sys::jobject) }
}

fn get_env() -> JNIEnv<'static> {
    unsafe { JNIEnv::from_raw(SDL_AndroidGetJNIEnv() as *mut _).unwrap() }
}
