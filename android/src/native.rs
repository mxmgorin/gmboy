use crate::JVM;
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use crate::file_dialog::{PICKED_DIR_URI, PICKED_FILE_URI};

/// Called from SDLActivity.onCreate or SDL2 main to store the JavaVM
#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_gmboy_MainActivity_nativeInit(env: JNIEnv, _class: JClass) {
    JVM.set(env.get_java_vm().unwrap()).ok();
}

/// This is the callback from Java when a directory is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_gmboy_MainActivity_nativeOnDirectoryPicked(
    env: JNIEnv,
    _class: JObject,
    uri: JString,
) {
    let uri_str = jstring_into_string(env, uri);
    *PICKED_DIR_URI.lock().unwrap() = Some(uri_str);
}

/// This is the callback from Java when a file is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_gmboy_MainActivity_nativeOnFilePicked(
    env: JNIEnv,
    _class: JObject,
    uri: JString,
) {
    let uri_str = jstring_into_string(env, uri);
    *PICKED_FILE_URI.lock().unwrap() = Some(uri_str);
}

fn jstring_into_string(mut env: JNIEnv, uri: JString) -> Option<String> {
    let result = env.get_string(&uri);

    match result {
        Ok(uri) => Some(uri.into()),
        Err(err) => {
            log::error!("Failed jstring_into_string: {err:?}");
            None
        }
    }
}
