//! All the work happens in `build.rs`, which compiles `../shader` to SPIR-V and
//! copies the result into `../assets/shaders/`. This binary just reports success.

fn main() {
    println!("shader built -> assets/shaders/solar_system.spv");
}
