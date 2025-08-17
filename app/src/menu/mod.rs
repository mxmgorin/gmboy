use crate::app::AppCmd;
use crate::config::AppConfig;

pub mod buffer;
pub mod files;
pub mod item;
pub mod factory;
pub mod menu;
pub mod roms;

pub use factory::*;
pub use menu::*;

pub const MAX_MENU_ITEMS_PER_PAGE: usize = 12;
pub const MAX_MENU_ITEM_CHARS: usize = 22;



pub fn get_menu_item_suffix(enabled: bool) -> &'static str {
    if enabled {
        "(●)"
    } else {
        ""
    }
}

pub trait SubMenu {
    fn get_iterator(&self) -> Box<dyn Iterator<Item = String> + '_>;
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self) -> Option<AppCmd>;
    fn move_right(&mut self) -> Option<AppCmd>;
    fn select(&mut self, _config: &AppConfig) -> (Option<AppCmd>, bool);
    fn next_page(&mut self);
    fn prev_page(&mut self);
}
