use std::arch::aarch64::int64x1_t;
use image::{ImageBuffer, RgbImage};
use rand::seq::SliceRandom;
use std::f64::consts::SQRT_2;
use rand::{Rng, thread_rng};

const WIDTH:u32 = 4096;
const HEIGHT:u32 = 2160;
const UNIT: f64 = 1.0/SQRT_2;
const RES: f64 = 1500.0;
const SMOOTHING_FUNCTION: SmoothingFunctions = SmoothingFunctions::Smooth;

struct Vecteur {
    x: f64,
    y: f64
}


enum SmoothingFunctions {
    Smooth,
    NoSmoothing,
}

fn apply_smoothing_function(number_to_smooth: &f64) -> f64 {
    match SMOOTHING_FUNCTION {
        SmoothingFunctions::Smooth => {
            let x = number_to_smooth;
            return 3.0 * x * x - 2.0 * x * x * x;
        },
        SmoothingFunctions::NoSmoothing => {
            return *number_to_smooth;
        }
    }

}

fn process_gradient_noise(permtable: &Vec<usize>, x: &f64, y: &f64) -> f64 {
    let gradient2 = [
        [UNIT, UNIT],
        [-UNIT, UNIT],
        [UNIT, -UNIT],
        [-UNIT, -UNIT],
        [1.0, 1.0],
        [-1.0, 1.0],
        [1.0, -1.0],
        [-1.0, -1.0]
    ];

    let resized_x = x / RES;
    let resized_y = y / RES;

    //On récupère les positions de la grille associée à (x,y)
    let x0 = resized_x as i32;
    let y0 = resized_y as i32;

    //Masquage
    let scaled_x = x0 % 256;
    let scaled_y = y0 % 256;

    let scaled_x: usize = scaled_x as usize;
    let scaled_y: usize = scaled_y as usize;

    let grad1 = permtable[scaled_x + permtable[scaled_y]] % 8;
    let grad2 = permtable[scaled_x + 1 + permtable[scaled_y]] % 8;
    let grad3 = permtable[scaled_x + permtable[scaled_y + 1]] % 8;
    let grad4 = permtable[scaled_x + 1 + permtable[scaled_y + 1]] % 8;

    // Coin en haut à gauche
    let dist_x = resized_x - f64::from(x0);
    let dist_y = resized_y - f64::from(y0);
    let scalar_product_top_left_corner = gradient2[grad1][0]*dist_x + gradient2[grad1][1] * dist_y;

    // Coin en haut à droite
    let dist_x = resized_x - (f64::from(x0) + 1.0);
    let dist_y = resized_y - f64::from(y0);
    let scalar_product_top_right_corner = gradient2[grad2][0]*dist_x + gradient2[grad2][1]*dist_y;

    // Coin en bas à gauche
    let dist_x = resized_x - (f64::from(x0));
    let dist_y = resized_y - (f64::from(y0) + 1.0);
    let scalar_product_bottom_left_corner = gradient2[grad3][0]*dist_x + gradient2[grad3][1]*dist_y;

    // Coin en bas à droite
    let dist_x = resized_x - (f64::from(x0) + 1.0);
    let dist_y = resized_y - (f64::from(y0) + 1.0);
    let scalar_product_bottom_right_corner = gradient2[grad4][0]*dist_x + gradient2[grad4][1]*dist_y;

    let smoothed_x = apply_smoothing_function(&(resized_x - f64::from(x0)));
    let smoothing_top = scalar_product_top_left_corner + smoothed_x * (scalar_product_top_right_corner - scalar_product_top_left_corner);
    let smoothing_bottom = scalar_product_bottom_left_corner + smoothed_x * (scalar_product_bottom_right_corner - scalar_product_bottom_left_corner);

    let smoothed_y = apply_smoothing_function(&(resized_y - f64::from(y0)));

    let interpolation = smoothing_top + smoothed_y * (smoothing_bottom-smoothing_top);

    return interpolation;
}

fn main() {
    // let mut perm: Vec<f64> = (0..256).map(|v| f64::from(v)).collect();
    let mut perm: Vec<usize> = (0..256).map(|v| v as usize).collect();
    let mut thread_rng = thread_rng();
    perm.shuffle(&mut thread_rng);
    let mut permtable: Vec<usize> = Vec::new();

    for i in 0..512 {
        permtable.push(perm[i%256]);
    }


    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    // let smoothing_function = SmoothingFunctions::NoSmoothing;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let generated_values_for_coordinates = ((process_gradient_noise(&permtable, &f64::from(x), &f64::from(y))+1.0)*0.5*255.0) as u8;
            // println!("{generated_values_for_coordinates}\n\n");
            *image.get_pixel_mut(x, y) = image::Rgb([
                generated_values_for_coordinates,
                generated_values_for_coordinates,
                generated_values_for_coordinates
            ]);
        }
    }

    // let mut image: RgbImage = ImageBuffer::new(5, 5);
    // *image.get_pixel_mut(3, 3) = image::Rgb([255,255,255]);
    // process_gradient_noise(&permtable, &260.5, &2.5, &100.0);
    image.save("output.png").unwrap();
}