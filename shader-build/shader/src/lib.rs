//! Rust port of `Bevy-Shader-Plugin/assets/shaders/solar_system.wgsl`.
//!
//! Bindings mirror the app's registrations in `../../../src/main.rs`:
//!   set 0, binding 0 — uniform: resolution
//!   set 1, binding 0 — storage (read): planet UV positions, re-sent every frame
//!   set 2, binding 0 — storage (read): per-planet colors, sparsely updated
//!
//! Layout note: every buffer type here is 16-byte-safe. `Vec2` arrays get an
//! 8-byte stride in a *storage* buffer under both rust-gpu's `repr(C)` rules and
//! `encase`'s WGSL rules, and `Vec4` arrays get 16. Adding a `Vec3` field, or an
//! `[f32; N]` inside the *uniform* buffer, would break that agreement.

#![no_std]

use spirv_std::glam::{Vec2, Vec4, vec2, vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

/// Planet count. Must match `N` in the app.
pub const N: usize = 8;

const STAR_CENTER: Vec2 = vec2(0.5, 0.5);
const STAR_RADIUS: f32 = 0.03;
const STAR_COLOR: Vec4 = vec4(1.0, 0.95, 0.4, 1.0);

/// Matches the app's `FrameUniform`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameUniform {
    pub resolution: Vec2,
}

/// WGSL's `fract`: `x - floor(x)`, always in `[0, 1)`.
///
/// Deliberately *not* Rust's `f32::fract`, which is `x - trunc(x)` and returns
/// negative values for negative inputs. `hash` below feeds `sin` output straight
/// into this, so half its inputs are negative — using the Rust semantics here
/// produces a visibly different star field from the WGSL original.
fn fract(x: f32) -> f32 {
    x - x.floor()
}

fn fract2(v: Vec2) -> Vec2 {
    vec2(fract(v.x), fract(v.y))
}

fn hash(p: Vec2) -> f32 {
    // WGSL: fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453)
    fract((p.x * 127.1 + p.y * 311.7).sin() * 43758.5453)
}

/// Brightness of a background star at the given pixel coordinate.
/// Works in pixel space so cells stay square regardless of aspect ratio.
fn star_field(pixel: Vec2) -> f32 {
    let cell = (pixel / 30.0).floor();
    let local = fract2(pixel / 30.0);
    if hash(cell) > 0.18 {
        return 0.0;
    }
    let pos = vec2(hash(cell + vec2(7.3, 2.1)), hash(cell + vec2(3.7, 8.5)));
    let brightness = 0.5 + 0.5 * hash(cell + vec2(5.0, 5.0));
    // WGSL: brightness * step(distance(local, pos), 0.06)
    // step(edge, x) is 1.0 when x >= edge, so this is `distance <= 0.06`.
    if local.distance(pos) <= 0.06 {
        brightness
    } else {
        0.0
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] u: &FrameUniform,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] positions: &[Vec2; N],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] colors: &[Vec4; N],
    output: &mut Vec4,
) {
    let frag = vec2(frag_coord.x, frag_coord.y);
    let uv = frag / u.resolution;

    // Star is always drawn on top.
    if uv.distance(STAR_CENTER) < STAR_RADIUS {
        *output = STAR_COLOR;
        return;
    }

    let mut i = 0usize;
    while i < N {
        let pos = positions[i];
        // Planet display radius scales with orbital distance.
        let planet_radius = 0.006 + 0.006 * pos.distance(STAR_CENTER);
        if uv.distance(pos) < planet_radius {
            let c = colors[i];
            *output = vec4(c.x, c.y, c.z, 1.0);
            return;
        }
        i += 1;
    }

    let bg = star_field(frag);
    *output = vec4(bg, bg, bg, 1.0);
}
