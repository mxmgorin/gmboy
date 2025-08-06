use app::AppFileDialog;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_dialog = TinyFileDialog;
    app::run(args, Box::new(file_dialog));
}

pub struct TinyFileDialog;

impl AppFileDialog for TinyFileDialog {
    fn select_file(&mut self, title: &str, filter: (&[&str], &str)) -> Option<String> {
        tinyfiledialogs::open_file_dialog(title, "", Some(filter))
    }

    fn select_dir(&mut self, title: &str) -> Option<String> {
        tinyfiledialogs::select_folder_dialog(title, "")
    }
}
