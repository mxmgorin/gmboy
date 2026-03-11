use crate::{
    get_roms_path, print_result_path,
    visual::util::{get_expected_path, run_visual_test, run_visual_test_dir},
};
use std::time::Duration;

mod util;

const DURATION: Duration = Duration::from_secs(1);
const IMG_UPDATE: bool = true;

#[test]
fn test_cgb_acid2() {
    let name = "cgb-acid2";
    let rom_path = get_roms_path().join(format!("{name}.gbc"));
    let img_path = get_expected_path().join(format!("{name}.png"));

    let result = run_visual_test(
        Some(core::emu::config::GbModel::Cgb),
        &rom_path,
        &img_path,
        IMG_UPDATE,
        DURATION,
    );

    if let Err(err) = result {
        panic!("{rom_path:?}: FAILED\n{err}")
    } else {
        println!("{rom_path:?}: OK");
    }
}

#[test]
fn test_dmg_acid2() {
    let name = "dmg-acid2";
    let rom_path = get_roms_path().join(format!("{name}.gb"));
    let img_path = get_expected_path().join(format!("{name}.png"));

    let result = run_visual_test(
        Some(core::emu::config::GbModel::Dmg),
        &rom_path,
        &img_path,
        IMG_UPDATE,
        DURATION,
    );

    if let Err(err) = result {
        panic!("{rom_path:?}: FAILED\n{err}")
    } else {
        println!("{rom_path:?}: OK");
    }
}

#[test]
fn test_acid_hell() {
    let name = "cgb-acid-hell";
    let rom_path = get_roms_path().join(name).join(format!("{name}.gbc"));
    let img_path = get_expected_path().join(format!("{name}.png"));

    let result = run_visual_test(
        Some(core::emu::config::GbModel::Cgb),
        &rom_path,
        &img_path,
        IMG_UPDATE,
        DURATION,
    );

    if let Err(err) = result {
        panic!("{rom_path:?}: FAILED\n{err}")
    } else {
        println!("{rom_path:?}: OK");
    }
}

#[test]
fn test_magen() {
    let path = get_roms_path().join("MagenTests");

    let results = run_visual_test_dir(
        Some(core::emu::config::GbModel::Cgb),
        &path,
        IMG_UPDATE,
        false,
        DURATION,
    );

    let mut failed = false;

    for (path, result) in results.into_iter() {
        failed = result.is_err();
        print_result_path(path, result);
    }

    assert!(!failed);
}
