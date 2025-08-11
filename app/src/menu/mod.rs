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

pub const MAX_MENU_ITEMS_PER_PAGE: usize = 10;
pub const MAX_MENU_ITEM_CHARS: usize = 22;

pub fn truncate_menu_item(s: &str) -> String {
    let max_len = s.len().min(MAX_MENU_ITEM_CHARS + 2);
    let mut truncated = String::with_capacity(max_len);

    for (i, ch) in s.chars().enumerate() {
        if i == MAX_MENU_ITEM_CHARS {
            let ends_with_paren = s.ends_with(')');
            let total_chars = s.chars().count();

            if total_chars > MAX_MENU_ITEM_CHARS + 1 || !ends_with_paren {
                truncated.push('…');
            }

            if ends_with_paren {
                truncated.push(')');
            }

            break;
        }

        truncated.push(ch);
    }

    truncated
}

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
