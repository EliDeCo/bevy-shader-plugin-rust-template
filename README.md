# solar-system-rustgpu

The `bevy-fragment-shader-plugin` solar system example, with the fragment shader
written in **Rust** (compiled to SPIR-V by [rust-gpu](https://github.com/Rust-GPU/rust-gpu))
instead of WGSL.

Visually identical to `Bevy-Shader-Plugin/examples/solar_system.rs` — same three
buffer types, same orbital simulation. Only the shader language differs.

## Running

Two steps, because the shader and the app build on different toolchains:

```sh
# 1. Compile the Rust shader to assets/shaders/solar_system.spv  (nightly)
cd shader-build
cargo build

# 2. Run the Bevy app  (stable)
cd ..
cargo run
```

Step 1 prints the entry point names it found:

```
warning: shader-build@0.1.0: entry points: ["main_fs"]
warning: shader-build@0.1.0: wrote .../assets/shaders/solar_system.spv
```

Re-run step 1 after editing the shader. It is incremental — `spirv-builder`
emits `rerun-if-changed` for the shader sources, so an unchanged shader is a
no-op.

The first build of step 1 is slow (several minutes): it compiles
`rustc_codegen_spirv` and the SPIR-V tools from source. Subsequent builds take
seconds.

### Expected output

On startup you will see three of these:

```
WARN naga::front::spv: Unknown decoration Block
```

They are harmless. naga's SPIR-V front end logs anything outside its decoration
enum and skips it — it infers block layout from the storage class instead. One
warning per buffer binding.

### Run with `cargo run`, not the bare executable

Bevy resolves `assets/` relative to `CARGO_MANIFEST_DIR` when it is set, and
relative to the executable otherwise. Launching `target/debug/solar-system-rustgpu.exe`
directly therefore fails with:

```
ERROR bevy_asset::server: Path not found: ...\target\debug\assets\shaders/solar_system.spv
```

Use `cargo run`, or copy `assets/` next to the executable when distributing.

## Layout

```
solar-system-rustgpu/
├── Cargo.toml              app — stable, workspace excludes shader-build
├── src/main.rs             the Bevy app
├── assets/shaders/         generated .spv lands here
└── shader-build/           ← everything nightly lives under here
    ├── rust-toolchain.toml nightly-2026-05-22
    ├── build.rs            runs SpirvBuilder, copies .spv into ../assets
    └── shader/             the no_std rust-gpu shader crate
        └── src/lib.rs      ← the actual shader
```

### Why the shader crate is nested inside `shader-build`

`spirv-builder` compiles the shader by spawning a second `cargo`, and it
deliberately strips `RUSTUP_TOOLCHAIN` from that child process. The inner cargo
therefore picks its toolchain purely by walking *up* from the shader crate's own
directory. Nesting `shader/` under `shader-build/` is what makes it find
`rust-toolchain.toml`.

Move `shader/` out to be a sibling and the inner build silently falls back to
stable, failing with `the -Z flag is only accepted on the nightly channel`.

### Why the split exists at all

`spirv-builder` depends on `rustc_codegen_spirv`, which needs
`feature(rustc_private)`. Cargo applies one toolchain per build, so any crate
that runs `SpirvBuilder` is pinned to rust-gpu's nightly. Keeping it in a
separate, workspace-excluded crate lets the Bevy app stay on stable and simply
load the generated `.spv` as an ordinary asset.

## How the bindings line up

`descriptor_set` → group index, `binding` → binding index. No glue code.

| `shader/src/lib.rs` | `src/main.rs` |
|---|---|
| `#[spirv(uniform, descriptor_set = 0, binding = 0)] &FrameUniform` | `register_uniform_buffer::<FrameUniform>(0, 0)` |
| `#[spirv(storage_buffer, descriptor_set = 1, binding = 0)] &[Vec2; 8]` | `register_storage_buffer::<PlanetPositions>(1, 0, false)` |
| `#[spirv(storage_buffer, descriptor_set = 2, binding = 0)] &[Vec4; 8]` | `register_array_buffer::<PlanetColors, Vec4, 8>(2, 0, false)` |

The app names the entry point explicitly:

```rust
FullscreenFragmentPlugin::new("shaders/solar_system.spv")
    .with_entry_point("main_fs")
```

`spirv-builder` packs every entry point in the shader crate into one `.spv`, so
adding a second `#[spirv(fragment)]` function without naming one here would fail
with `MultipleEntryPointsFound`.

## Two gotchas worth knowing

**`fract` is not `f32::fract`.** WGSL's `fract(x)` is `x - floor(x)` and always
returns `[0, 1)`. Rust's `f32::fract` is `x - trunc(x)` and goes negative for
negative inputs. The `hash` function feeds `sin` output straight into it, so half
the inputs are negative — using the Rust version changes the star field. See the
hand-written `fract` in `shader/src/lib.rs`.

**Buffer layouts must be 16-byte-friendly.** rust-gpu derives offsets from
`#[repr(C)]`, the CPU side (`encase`) uses WGSL rules. They agree for everything
used here, but a `Vec3` field or an `[f32; N]` in a *uniform* buffer would
diverge. Both failure modes are loud rather than silent — the array case is
rejected by rust-gpu's own validator at compile time, the `Vec3` case by naga at
pipeline creation.

## Requirements

- `rustup toolchain install nightly-2026-05-22` with the `rust-src`, `rustc-dev`
  and `llvm-tools` components (the `rust-toolchain.toml` requests these).
- A Vulkan, Metal or DX12 backend. This path does **not** work on the web:
  browsers accept WGSL only.
