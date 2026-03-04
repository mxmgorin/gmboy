use crate::{gbmicrotest::utils::run_gbmicrotest_dir_roms, print_result_path};

mod utils;

#[ignore]
#[test]
fn test_all() {
    let results = run_gbmicrotest_dir_roms(10000, 0, true);
    let mut failed = false;

    for (_, (path, result)) in results.into_iter().enumerate() {
        failed = result.is_err();
        print_result_path(path, result);
        assert!(!failed);
    }

    assert!(!failed);
}
