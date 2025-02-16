mod blargg;
mod sm83;
mod mooneye;

pub fn print_with_dashes(content: &str) {
    const TOTAL_LEN: usize = 100;
    let content_length = content.len();
    let dashes = "-".repeat(TOTAL_LEN.saturating_sub(content_length));
    println!("{} {}", content, dashes);
}
