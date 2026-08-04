# solar-system-rustgpu

An example for [bevy-fragment-shader-plugin](https://github.com/EliDeCo/Bevy-Shader-Plugin) with the fragment shader written in Rust using [rust-gpu](https://github.com/Rust-GPU/rust-gpu) instead of WGSL.

## Requirements

- `rustup toolchain install nightly-2026-05-22` with the `rust-src`, `rustc-dev` and `llvm-tools` components
- Vulkan, Metal or DX12. Does not work on web.

## Running

```sh
cd shader-build
cargo build       # builds the shader (nightly)

cd ..
cargo run         # runs the app (stable)
```

Re-run `cargo build` in `shader-build/` after editing the shader. The first one takes a few minutes, later ones a few seconds.

Use `cargo run` rather than launching the executable directly, otherwise Bevy won't find `assets/`.

`WARN naga::front::spv: Unknown decoration Block` on startup is harmless.

## Layout

```
Cargo.toml              app (stable)
src/main.rs
assets/shaders/         generated .spv
shader-build/           nightly
  rust-toolchain.toml
  build.rs              builds the shader, copies the .spv into ../assets
  shader/src/lib.rs     the shader
```

Keep the shader crate inside `shader-build/`. As a sibling it builds with the wrong toolchain and fails with `the -Z flag is only accepted on the nightly channel`.

## Bindings

`descriptor_set` is the group index, `binding` is the binding index.

| shader | app |
|---|---|
| `#[spirv(uniform, descriptor_set = 0, binding = 0)] &FrameUniform` | `register_uniform_buffer::<FrameUniform>(0, 0)` |
| `#[spirv(storage_buffer, descriptor_set = 1, binding = 0)] &[Vec2; 8]` | `register_storage_buffer::<PlanetPositions>(1, 0, false)` |
| `#[spirv(storage_buffer, descriptor_set = 2, binding = 0)] &[Vec4; 8]` | `register_array_buffer::<PlanetColors, Vec4, 8>(2, 0, false)` |

Name the entry point — it's the shader function's name:

```rust
FullscreenFragmentPlugin::new("shaders/solar_system.spv")
    .with_entry_point("main_fs")
```

## Buffer layout

Stick to `Vec2`, `Vec4`, `f32` and arrays of them. Avoid `Vec3`. Keep arrays out of uniform buffers unless the element is 16 bytes — use a storage buffer instead.
