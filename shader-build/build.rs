use spirv_builder::SpirvBuilder;
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let project = manifest.parent().unwrap();

    // The shader crate lives *inside* this one so that rustup, walking up from
    // it, finds our rust-toolchain.toml. spirv-builder strips RUSTUP_TOOLCHAIN
    // from the inner cargo invocation, so directory position is what selects the
    // nightly — a sibling directory would silently fall back to stable.
    let mut builder = SpirvBuilder::new(manifest.join("shader"), "spirv-unknown-vulkan1.2");
    // Forwards rustc warnings/errors from the shader crate and emits
    // rerun-if-changed for its sources.
    builder.build_script.defaults = true;

    let result = builder.build()?;

    let dest = project.join("assets/shaders/solar_system.spv");
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::copy(result.module.unwrap_single(), &dest)?;

    // Surfaced as build warnings so they show up in normal cargo output. The
    // entry point list is what you pass to `with_entry_point` in the app.
    println!("cargo::warning=entry points: {:?}", result.entry_points);
    println!("cargo::warning=wrote {}", dest.display());

    Ok(())
}
