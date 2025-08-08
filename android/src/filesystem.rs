use crate::get_env;
use crate::java::{
    get_file_name, read_uri_bytes, show_android_directory_picker, show_android_file_picker,
};
use app::AppFilesystem;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub static PICKED_FILE_URI: Mutex<Option<String>> = Mutex::new(None);
pub static PICKED_DIR_URI: Mutex<Option<String>> = Mutex::new(None);
const TIMEOUT: Duration = Duration::from_secs(120);

pub struct AndroidFilesystem;

impl AndroidFilesystem {
    pub fn wait(v: &Mutex<Option<String>>) -> Option<String> {
        let start = Instant::now();

        loop {
            if let Some(uri) = v.lock().unwrap().take() {
                return Some(uri);
            }

            if start.elapsed() > TIMEOUT {
                return None;
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

impl AppFilesystem for AndroidFilesystem {
    fn select_file(&mut self, _title: &str, _filter: (&[&str], &str)) -> Option<String> {
        log::info!("select_file");
        let mut env = get_env();
        show_android_file_picker(&mut env);

        let uri = AndroidFilesystem::wait(&PICKED_FILE_URI);
        log::debug!("select_file: {uri:?}");

        uri
    }

    fn select_dir(&mut self, _title: &str) -> Option<String> {
        log::info!("select_dir");
        let mut env = get_env();
        show_android_directory_picker(&mut env);

        let uri = AndroidFilesystem::wait(&PICKED_DIR_URI);
        log::debug!("select_dir: {uri:?}");

        uri
    }

    fn get_file_name(&self, path: &Path) -> Option<String> {
        let path = path.to_str()?;

        get_file_name(path)
    }

    fn read_file_bytes(&self, path: &Path) -> Option<Box<[u8]>> {
        let path = path.to_str()?;

        read_uri_bytes(path).map(|x| x.into_boxed_slice())
    }
}
