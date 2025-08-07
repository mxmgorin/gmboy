use crate::filesystem::{PICKED_DIR_URI, PICKED_FILE_URI};
use crate::JVM;
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;

/// Called from SDLActivity.onCreate or SDL2 main to store the JavaVM
#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_gmboy_MainActivity_nativeInit(env: JNIEnv, _class: JClass) {
    JVM.set(env.get_java_vm().unwrap()).ok();
}

/// This is the callback from Java when a directory is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_gmboy_MainActivity_nativeOnDirectoryPicked(
    mut env: JNIEnv,
    _class: JObject,
    uri: JString,
) {
    let uri_str: String = env.get_string(&uri).unwrap().into();
    *PICKED_DIR_URI.lock().unwrap() = Some(uri_str);
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
