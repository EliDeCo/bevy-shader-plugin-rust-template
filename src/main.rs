//! A fullscreen fragment shader written in Rust. The shader is in `shader/src/lib.rs` and is
//! rebuilt by `build.rs` whenever it changes, so `cargo run` is all you need.

use bevy::prelude::*;
use bevy_shader_plugin::prelude::*;

// Written by build.rs.
const SHADER_PATH: &str = "shaders/shader.spv";
// rust-gpu names entry points after their functions.
const FRAGMENT_ENTRY: &str = "main_fs";

// group(0) binding(0). Must match `Globals` in the shader.
#[derive(Resource, ShaderType, Clone, Default)]
struct Globals {
    resolution: Vec2,
    time: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FullscreenFragmentPlugin::new(SHADER_PATH).with_entry_point(FRAGMENT_ENTRY))
        .register_uniform_buffer::<Globals>(0, 0)
        .init_resource::<Globals>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_globals)
        .run();
}

fn setup(mut commands: Commands) {
    // The plugin draws through Bevy's 3D pipeline and doesn't support MSAA.
    commands.spawn((Camera3d::default(), Msaa::Off));
}

fn update_globals(
    mut globals: ResMut<Globals>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = windows.single() else { return };
    globals.resolution = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );
    globals.time = time.elapsed_secs();
}
