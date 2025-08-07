use app::AppFilesystem;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_dialog = DesktopFileSystem;
    app::run(args, Box::new(file_dialog));
}

pub struct DesktopFileSystem;

impl AppFilesystem for DesktopFileSystem {
    fn select_file(&mut self, title: &str, filter: (&[&str], &str)) -> Option<String> {
        tinyfiledialogs::open_file_dialog(title, "", Some(filter))
    }

    fn select_dir(&mut self, title: &str) -> Option<String> {
        tinyfiledialogs::select_folder_dialog(title, "")
    }

    fn get_file_name(&self, path: &Path) -> Option<String> {
        path.file_stem()?.to_str().map(|x| x.to_string())
    }

    fn read_file_bytes(&self, path: &Path) -> Option<Box<[u8]>> {
        core::read_bytes(path).ok()
    }
}
