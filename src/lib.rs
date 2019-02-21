use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    x: f64,
    y: f64,
    z: f64,
) -> Result<(), JsValue> {
    let mut data = lorenz_attractor(width, height, x, y, z);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn lorenz_attractor(width: u32, height: u32, x: f64, y: f64, z: f64) -> Vec<u8> {
    let mut data = Vec::new();

    for i in 0..width {
        for j in 0..height {
            data.push(100);
            data.push(100);
            data.push(100);
            data.push(255);
        }
    }

    data
}

fn lorenz_solver(x_0: f64, y_0: f64, z_0: f64, iterations: usize, dt: f64) -> ( Vec<f64>, Vec<f64>, Vec<f64> ) {
    let mut x_path = Vec::with_capacity(iterations);
    let mut y_path = Vec::with_capacity(iterations);
    let mut z_path = Vec::with_capacity(iterations);

    let mut x_i: f64;
    let mut y_i: f64;
    let mut z_i: f64;
    //
    // constants
    let a = 10_f64;
    let b = 28_f64;
    let numerator = 8_f64;
    let denominator = 3_f64;
    let r: f64;
    r = numerator/denominator;

    // starting location
    x_path.push(x_0);
    y_path.push(y_0);
    z_path.push(z_0);
    for i in 1..iterations {
        // discretised lorenz equations
        x_i = x_path[i-1] + a * (x_path[i-1] - y_path[i-1]) * dt;
        y_i = y_path[i-1] + (r * x_path[i-1] - x_path[i-1] * z_path[i-1] - y_path[i-1]) * dt;
        z_i = z_path[i-1] + (x_path[i-1] * y_path[i-1] - b * z_path[i-1]) * dt;

        // add into the solution vectors
        x_path.push(x_i);
        y_path.push(y_i);
        z_path.push(z_i);
    }
    (x_path, y_path, z_path)
}
