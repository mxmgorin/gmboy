use crate::{get_activity, JVM};
use jni::objects::{JByteArray, JObject, JString, JValue};
use jni::JNIEnv;

/// Call this to show the file picker
pub fn show_android_file_picker(env: &mut JNIEnv) {
    let activity = get_activity();
    env.call_method(activity, "openFilePicker", "()V", &[])
        .expect("Failed to call openFilePicker");
}

/// Call this to show the directory picker
pub fn show_android_directory_picker(env: &mut JNIEnv) {
    let activity = get_activity();
    env.call_method(activity, "openDirectoryPicker", "()V", &[])
        .expect("Failed to call openDirectoryPicker");
}

/// Reads bytes from an Android content:// URI using Java's ContentResolver
pub fn read_uri_bytes(uri: &str) -> Option<Vec<u8>> {
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
        .ok()?;

    if result_obj.is_null() {
        return None;
    }

    let result_array: JByteArray = JByteArray::from(result_obj);

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
