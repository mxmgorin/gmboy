#[no_mangle]
pub extern "C" fn Java_com_mxmgorin_gmboy_MainActivity_nativeMain(_: *const (), _: *const ()) {
    app::run(vec![]);
}
