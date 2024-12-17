mod perlin;
use std::f64::consts::SQRT_2;

const WIDTH:u32 = 4096;
const HEIGHT:u32 = 2160;
const UNIT: f64 = 1.0/SQRT_2;
const RES: f64 = 1500.0;


fn main() {
    for i in 0..3 {
        perlin::perlin_noise::generate_image(format!("generated_{i}.png").as_str(), &WIDTH, &HEIGHT, &RES);
    }
}