const PI: f64 = 3.14159265358979323846;

pub fn vanilla_table() -> [f32; 65536] {
    let mut table = [0.0f32; 65536];
    for i in 0..table.len() {
        table[i] = ((i as f64) * PI * 2.0 / 65536.0).sin() as f32;
    }
    table
}

pub fn old_fast_table() -> [f32; 4096] {
    let mut table = [0.0f32; 4096];
    for i in 0..table.len() {
        table[i] = (((i as f32 + 0.5) / 4096.0 * (PI as f32 * 2.0)) as f64).sin() as f32;
    }
    for i in (0..360).step_by(90) {
        table[(i as f32 * 11.377778) as usize & 4095] = ((i as f32 * 0.017453292) as f64).sin() as f32;
    }
    table
}

pub fn new_fast_table() -> [f32; 4096] {
    let mut table = [0.0f32; 4096];
    for i in 0..table.len() {
        table[i] = (i as f64 * PI * 2.0 / 4096.0).sin() as f32;
    }
    table
}

fn weird_round(val: f64) -> f32 {
    ((val * 1.0e8).round() / 1.0e8) as f32
}

pub fn vanilla_sin_index(radians: f32) -> usize {
    (radians * 10430.378) as usize & 65535
}
pub fn vanilla_cos_index(radians: f32) -> usize {
    (radians * 10430.378 + 16384.0) as usize & 65535
}

pub fn old_fast_sin_index(radians: f32) -> usize {
    (radians * 651.8986) as usize & 4095
}
pub fn old_fast_cos_index(radians: f32) -> usize {
    ((radians + (PI as f32 / 2.0)) * 651.8986) as usize & 4095
}

pub fn new_fast_sin_index(radians: f32) -> usize {
    (radians * 651.8986) as usize & 4095
}
pub fn new_fast_cos_index(radians: f32) -> usize {
    (radians * 651.8986 + 1024.0) as usize & 4095
}

pub fn mc_radians(degrees: f32) -> f32 {
    degrees * (PI as f32) / 180.0
}