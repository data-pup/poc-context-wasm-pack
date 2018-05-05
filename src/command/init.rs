use std::time::Instant;
use std::result;
use indicatif::HumanDuration;

use error::Error;

use bindgen;
use emoji;
use manifest;
use readme;
use PBAR;

use super::create_pkg_dir;
use super::build::{cargo_build_wasm, rustup_add_wasm_target};

pub fn init(crate_path: &str, scope: Option<String>) -> result::Result<(), Error> {
    let started = Instant::now();

    rustup_add_wasm_target()?;
    cargo_build_wasm(&crate_path)?;
    create_pkg_dir(&crate_path)?;
    manifest::write_package_json(&crate_path, scope)?;
    readme::copy_from_crate(&crate_path)?;
    bindgen::cargo_install_wasm_bindgen()?;
    let name = manifest::get_crate_name(&crate_path)?;
    bindgen::wasm_bindgen_build(&crate_path, &name)?;
    PBAR.message(&format!(
        "{} Done in {}",
        emoji::SPARKLE,
        HumanDuration(started.elapsed())
    ));
    PBAR.message(&format!(
        "{} Your WASM pkg is ready to publish at {}/pkg",
        emoji::PACKAGE,
        &crate_path
    ));
    Ok(())
}
