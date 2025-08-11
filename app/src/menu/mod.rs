pub mod menu;
pub mod item;
pub mod files;
pub mod roms;
pub mod buffer;
mod list;

pub use menu::*;
pub use list::*;

pub const MAX_ROMS_PER_PAGE: usize = 10;
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

fn get_menu_item_suffix(enabled: bool) -> &'static str {
    if enabled {
        "(●)"
    } else {
        ""
    }
}
