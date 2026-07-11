// Embeds the oxGBC icon and version metadata into oxgbc.exe so Windows shows the
// brand icon in Explorer, the taskbar, and the window, plus proper fields under
// File > Properties > Details. No-op on every other platform (the winresource
// build-dependency and this code are both gated to Windows hosts in Cargo.toml).
fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        // Path is relative to this crate's manifest dir (build script cwd).
        res.set_icon("../../media/oxgbc.ico");
        res.set("ProductName", "oxGBC");
        res.set("FileDescription", "oxGBC — Game Boy / Game Boy Color emulator");
        res.set("LegalCopyright", "Licensed under GPL-3.0");
        res.compile().expect("failed to embed Windows resources");
    }
}
