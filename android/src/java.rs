use crate::{get_activity, get_env, JVM};
use jni::objects::{JByteArray, JObject, JString, JValue};
use jni::AttachGuard;

const CLASS_NAME: &str = "com/mxmgorin/gmboy/MainActivity";

/// Call this to show the file picker
pub fn show_android_file_picker() {
    let mut env = get_env();
    let activity = get_activity();

    env.call_method(activity, "openFilePicker", "()V", &[])
        .expect("Failed to call openFilePicker");
}

/// Call this to show the directory picker
pub fn show_android_directory_picker() {
    let mut env = get_env();
    let activity = get_activity();

    env.call_method(activity, "openDirectoryPicker", "()V", &[])
        .expect("Failed to call openDirectoryPicker");
}

/// Reads bytes from an Android content:// URI using Java's ContentResolver
pub fn read_uri_bytes(uri: &str) -> Option<Vec<u8>> {
    let vm = JVM.get()?;
    let mut env = vm.attach_current_thread().ok()?;
    let activity_class = env.find_class(CLASS_NAME).ok()?;

    let j_str: JString = env.new_string(uri).ok()?;
    let j_obj = JObject::from(j_str);
    let j_val = JValue::Object(&j_obj);

    let j_obj = env
        .call_static_method(
            activity_class,
            "readUriBytes",
            "(Ljava/lang/String;)[B",
            &[j_val],
        )
        .ok()?
        .l()
        .ok()?;

    if j_obj.is_null() {
        return None;
    }

    env.convert_byte_array(JByteArray::from(j_obj)).ok()
}

pub fn get_file_name(uri: &str) -> Option<String> {
    let vm = JVM.get()?;
    let mut env = vm.attach_current_thread().ok()?;
    let activity_class = env.find_class(CLASS_NAME).ok()?;

    let j_str = env.new_string(uri).ok()?;
    let j_obj = JObject::from(j_str);
    let j_val = JValue::Object(&j_obj);

    let j_obj = env
        .call_static_method(
            activity_class,
            "getFileName",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[j_val],
        )
        .ok()?
        .l()
        .ok()?;

    if j_obj.is_null() {
        return None;
    }

    let filename = env.get_string(&JString::from(j_obj)).ok()?.into();

    Some(filename)
}

/// Call Java method getFilesInDirectory(String uri, String[] extensions)
pub fn read_dir(uri: &str) -> Option<Vec<String>> {
    let vm = JVM.get()?;
    let mut env = vm.attach_current_thread().ok()?;
    let class = env.find_class(CLASS_NAME).unwrap();

    let j_uri = env.new_string(uri).unwrap();
    let j_obj = env
        .call_static_method(
            class,
            "getFilesInDirectory",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[JValue::Object(&JObject::from(j_uri))],
        )
        .unwrap()
        .l()
        .unwrap();

    j_obj_to_vec(&mut env, j_obj)
}

fn j_obj_to_vec(env: &mut AttachGuard, list: JObject) -> Option<Vec<String>> {
    let size = env
        .call_method(&list, "size", "()I", &[])
        .unwrap()
        .i()
        .unwrap_or(8);
    let mut vec = Vec::with_capacity(size as usize);

    for i in 0..size {
        let element = env
            .call_method(&list, "get", "(I)Ljava/lang/Object;", &[JValue::from(i)])
            .unwrap()
            .l()
            .unwrap();

        let string = env.get_string(&JString::from(element)).unwrap().into();
        vec.push(string);
    }

    Some(vec)
}
