use crate::file_dialog::{PICKED_DIR_URI, PICKED_FILE_URI};
use crate::JVM;
use jni::errors::LogErrorAndDefault;
use jni::objects::{JClass, JObject, JString};
use jni::EnvUnowned;

/// Called from SDLActivity.onCreate or SDL2 main to store the JavaVM
#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_oxgbc_MainActivity_nativeInit<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    env.with_env(|env| -> jni::errors::Result<()> {
        JVM.set(env.get_java_vm()?).ok();
        Ok(())
    })
    .resolve::<LogErrorAndDefault>();
}

/// This is the callback from Java when a directory is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_oxgbc_MainActivity_nativeOnDirectoryPicked<'local>(
    mut env: EnvUnowned<'local>,
    _class: JObject<'local>,
    uri: JString<'local>,
) {
    let uri_str = env
        .with_env(|env| -> jni::errors::Result<Option<String>> {
            Ok(jstring_into_string(env, &uri))
        })
        .resolve::<LogErrorAndDefault>();
    *PICKED_DIR_URI.lock().unwrap() = Some(uri_str);
}

/// This is the callback from Java when a file is picked
#[no_mangle]
pub extern "system" fn Java_com_mxmgorin_oxgbc_MainActivity_nativeOnFilePicked<'local>(
    mut env: EnvUnowned<'local>,
    _class: JObject<'local>,
    uri: JString<'local>,
) {
    let uri_str = env
        .with_env(|env| -> jni::errors::Result<Option<String>> {
            Ok(jstring_into_string(env, &uri))
        })
        .resolve::<LogErrorAndDefault>();
    *PICKED_FILE_URI.lock().unwrap() = Some(uri_str);
}

fn jstring_into_string(env: &jni::Env, uri: &JString) -> Option<String> {
    match uri.try_to_string(env) {
        Ok(uri) => Some(uri),
        Err(err) => {
            log::error!("Failed jstring_into_string: {err:?}");
            None
        }
    }
}
