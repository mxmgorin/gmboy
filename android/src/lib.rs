use core::cart::Cart;
use app::AppFileDialog;
use jni::objects::{JByteArray, JObject, JString};
use jni::JNIEnv;
use std::backtrace::Backtrace;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[no_mangle]
pub extern "C" fn SDL_main(_argc: i32, _argv: *const *const u8) -> i32 {
    log("SDL_main entered");

    std::panic::set_hook(Box::new(|info| {
        let bt = Backtrace::capture();
        log(&format!("Rust panic: {info}\nBacktrace:\n{bt:?}"));
    }));

    _ = std::panic::catch_unwind(|| {
        let args: Vec<String> = std::env::args().collect();
        let file_dialog = JavaFileDialog;
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

static PICKED_FILE_URI: Mutex<Option<String>> = Mutex::new(None);

fn get_activity<'a>() -> JObject<'a> {
    unsafe { JObject::from_raw(SDL_AndroidGetActivity() as jni::sys::jobject) }
}

/// Call this to show the file picker
pub fn show_android_file_picker(env: &mut JNIEnv) {
    let activity = get_activity();
    env.call_method(activity, "openFilePicker", "()V", &[])
        .expect("Failed to call openFilePicker");
}

/// This is the callback from Java when a file is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_gmboy_MainActivity_nativeOnFilePicked(
    mut env: JNIEnv,
    _class: JObject,
    uri: JString,
) {
    log("Java_com_mxmgorin_MainActivity_nativeOnFilePicked");
    let uri_str: String = env.get_string(&uri).unwrap().into();
    log(&format!("{uri_str}"));

    *PICKED_FILE_URI.lock().unwrap() = Some(uri_str);
}

/// Get last picked file URI
pub fn get_picked_file_uri() -> Option<String> {
    PICKED_FILE_URI.lock().unwrap().clone()
}

pub struct JavaFileDialog;

impl AppFileDialog for JavaFileDialog {
    fn select_file(&mut self, _title: &str, _filter: (&[&str], &str)) -> Option<String> {
        log("select_file");
        let mut env = get_env();
        show_android_file_picker(&mut env);
        let start = Instant::now();

        loop {
            if let Some(uri) = get_picked_file_uri() {
                if let Some(bytes) = read_file_bytes(&uri) {
                    if let Ok(cart) = Cart::new(bytes.into_boxed_slice()) {
                        log(&format!("Selected cart: {}", cart.data.get_title()));
                    }
                }

                return Some(uri);
            }

            if start.elapsed() > Duration::from_secs(120) {
                return None;
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn select_dir(&mut self, _title: &str) -> Option<String> {
        None
    }
}

fn get_env() -> JNIEnv<'static> {
    unsafe { JNIEnv::from_raw(SDL_AndroidGetJNIEnv() as *mut _).unwrap() }
}

/// Read file bytes using Java ContentResolver
pub fn read_file_bytes(uri: &str) -> Option<Vec<u8>> {
    let mut env = get_env();
    let activity = get_activity();

    let j_uri = env.new_string(uri).unwrap();
    let result = env
        .call_method(
            activity,
            "readUriBytes",
            "(Ljava/lang/String;)[B",
            &[(&JObject::from(j_uri)).into()],
        )
        .ok()?;

    let byte_array = result.l().ok()?;
    if byte_array.is_null() {
        return None;
    }

    let byte_array: JByteArray = JByteArray::from(byte_array);
    let len = env.get_array_length(&byte_array).ok()? as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(byte_array, 0, &mut buf).ok()?;

    // Convert i8 -> u8
    Some(buf.into_iter().map(|b| b as u8).collect())
}
