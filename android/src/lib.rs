#[no_mangle]
pub extern "C" fn Main(_: *const (), _: *const ()) {
    app::run(vec![]);
}