use app::AppFileSystem;
use jni::objects::{JByteArray, JClass, JObject, JString, JValue};
use jni::{JNIEnv, JavaVM};
use std::backtrace::Backtrace;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
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
        let file_dialog = AndroidFileSystem;
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

static JVM: OnceLock<JavaVM> = OnceLock::new();

/// Called from SDLActivity.onCreate or SDL2 main to store the JavaVM
#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_gmboy_MainActivity_nativeInit(env: JNIEnv, _class: JClass) {
    JVM.set(env.get_java_vm().unwrap()).ok();
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
    let uri_str: String = env.get_string(&uri).unwrap().into();

    *PICKED_FILE_URI.lock().unwrap() = Some(uri_str);
}

/// Get last picked file URI
pub fn get_picked_file_uri() -> Option<String> {
    PICKED_FILE_URI.lock().unwrap().clone()
}

pub struct AndroidFileSystem;

impl AppFileSystem for AndroidFileSystem {
    fn select_file(&mut self, _title: &str, _filter: (&[&str], &str)) -> Option<String> {
        log("select_file");
        let mut env = get_env();
        show_android_file_picker(&mut env);
        let start = Instant::now();

        loop {
            if let Some(uri) = get_picked_file_uri() {
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

    fn get_file_name(&self, path: &Path) -> Option<String> {
        let path = path.to_str()?;

        get_file_name(path)
    }

    fn read_file_bytes(&self, path: &Path) -> Option<Box<[u8]>> {
        let path = path.to_str()?;

        read_content_uri(path).map(|x| x.into_boxed_slice())
    }
}

fn get_env() -> JNIEnv<'static> {
    unsafe { JNIEnv::from_raw(SDL_AndroidGetJNIEnv() as *mut _).unwrap() }
}

/// Reads bytes from an Android content:// URI using Java's ContentResolver
pub fn read_content_uri(uri: &str) -> Option<Vec<u8>> {
    let vm = JVM.get()?;
    let mut env = vm.attach_current_thread().ok()?;

    let activity_class = env.find_class("com/mxmgorin/gmboy/MainActivity").ok()?;
    let juri: JString = env.new_string(uri).ok()?;
    let obj = JObject::from(juri);
    let arg = JValue::Object(&obj);

    // Call Java static method readUriBytes(String) -> byte[]
    let result_obj = env
        .call_static_method(
            activity_class,
            "readUriBytes",
            "(Ljava/lang/String;)[B",
            &[arg],
        )
        .ok()?
        .l()
        .ok()?; // JObject

    if result_obj.is_null() {
        return None;
    }

    // Wrap into JByteArray
    let result_array: JByteArray = JByteArray::from(result_obj);

    // Convert to Vec<u8>
    env.convert_byte_array(result_array).ok()
}

pub fn get_file_name(uri: &str) -> Option<String> {
    let vm = JVM.get()?;
    let mut env = vm.attach_current_thread().ok()?;

    let activity_class = env.find_class("com/mxmgorin/gmboy/MainActivity").ok()?;
    let juri = env.new_string(uri).ok()?;
    let obj = JObject::from(juri);
    let arg = JValue::Object(&obj);

    let jstr = env
        .call_static_method(
            activity_class,
            "getFileName",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[arg],
        )
        .ok()?
        .l()
        .ok()?;

    if jstr.is_null() {
        return None;
    }

    let filename: String = env.get_string(&JString::from(jstr)).ok()?.into();

    Some(filename)
}
