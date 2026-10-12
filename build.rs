use spirv_builder::SpirvBuilder;
use std::{env, fs, path::PathBuf};

// Compiles the shader crate in `shader/` to SPIR-V and copies it into `assets/`, where the
// app loads it. Cargo re-runs this whenever the shader's source changes.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let mut builder = SpirvBuilder::new(manifest.join("shader"), "spirv-unknown-vulkan1.4");
    builder.build_script.defaults = true;
    let result = builder.build()?;

    let dest = manifest.join("assets/shaders/shader.spv");
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::copy(result.module.unwrap_single(), &dest)?;

    Ok(())
}
