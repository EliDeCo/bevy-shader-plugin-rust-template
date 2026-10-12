//! The fragment shader drawn by `src/main.rs`.
//!
//! `descriptor_set` is the group index and `binding` the binding index passed to the
//! `register_*` functions in the app.

#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]

use spirv_std::glam::{Vec2, Vec4, vec2, vec4};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

// Must match `Globals` in the app.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Globals {
    pub resolution: Vec2,
    pub time: f32,
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] globals: &Globals,
    output: &mut Vec4,
) {
    let uv = vec2(frag_coord.x, frag_coord.y) / globals.resolution;
    *output = vec4(uv.x, uv.y, 0.5 + 0.5 * globals.time.sin(), 1.0);
}
