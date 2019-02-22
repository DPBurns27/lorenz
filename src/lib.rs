use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    x_0: f64,
    y_0: f64,
    z_0: f64,
) -> Result<(), JsValue> {
    let mut data = lorenz_attractor(width as usize, height as usize, x_0, y_0, z_0);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn lorenz_attractor(width: usize, height: usize, x_0: f64, y_0: f64, z_0: f64) -> Vec<u8> {

    let canvas_vector_size: usize = width * height * 4;
    let mut data = Vec::with_capacity(canvas_vector_size);


    // initialise vector values
    for i in 0..width {
        for j in 0..height {
            data.push(100);
            data.push(100);
            data.push(100);
            data.push(255);
        }
    }

    // domain definition
    let domain_x_min: f64 = -20.0;
    let domain_x_max: f64 = 20.0;
    let domain_z_min: f64 = 0.0;
    let domain_z_max: f64 = 40.0;
    let domain_x_interval: f64 = (domain_x_max - domain_x_min)/(width as f64);
    let domain_z_interval: f64 = (domain_z_max - domain_z_min)/(height as f64);

    for xiter in (-20..20).step_by(4) {
        for yiter in (-20..20).step_by(4) {
            for ziter in (0..40).step_by(4) {
                 
                let (x_path, y_path, z_path) = lorenz_solver(xiter as f64, yiter as f64, ziter as f64, 5000, 0.001);

                for i in 0..x_path.len() {
        
                    let pixel_i: usize = ((x_path[i]/domain_x_interval) - domain_x_min/domain_x_interval) as usize;        
                    let pixel_j: usize = (z_path[i]/domain_z_interval) as usize;        
        
                    pixel(pixel_i, pixel_j, 200, 200, 200, width, height, &mut data);
                }
            }
        }
    }

    data
}

fn pixel(i: usize, j: usize, r: u8, g: u8, b: u8, width: usize, height: usize, canvas: &mut Vec<u8>) {

    if i < width || j < height {
        let canvas_address: usize = ((j) * width + i) * 4; 
        canvas[canvas_address] = r;
        canvas[canvas_address + 1] = g;
        canvas[canvas_address + 2] = b;
    }
}

fn lorenz_solver(x_0: f64, y_0: f64, z_0: f64, iterations: usize, dt: f64) -> ( Vec<f64>, Vec<f64>, Vec<f64> ) {
    eprintln!("im in the solver");
    let mut x_path = Vec::with_capacity(iterations);
    let mut y_path = Vec::with_capacity(iterations);
    let mut z_path = Vec::with_capacity(iterations);

    let mut x_i: f64;
    let mut y_i: f64;
    let mut z_i: f64;
    //
    // constants
    let a = 10_f64;
    let r = 28_f64;
    let numerator = 8_f64;
    let denominator = 3_f64;
    let b: f64;
    b = numerator/denominator;

    // starting location
    x_path.push(x_0);
    y_path.push(y_0);
    z_path.push(z_0);
    for i in 1..iterations {
        // discretised lorenz equations
        x_i = x_path[i-1] + a * (x_path[i-1] - y_path[i-1]) * dt;
        y_i = y_path[i-1] + (r * x_path[i-1] - x_path[i-1] * z_path[i-1] - y_path[i-1]) * dt;
        z_i = z_path[i-1] + (x_path[i-1] * y_path[i-1] - b * z_path[i-1]) * dt;

        if x_i >= 20.0 || x_i <= -20.0 || y_i <= -20.0 || y_i >= 20.0 ||  z_i <= 0.0 || z_i >= 40.0 {           
            break;
        }

        // add into the solution vectors
        x_path.push(x_i);
        y_path.push(y_i);
        z_path.push(z_i);
    }

    (x_path, y_path, z_path)
}
