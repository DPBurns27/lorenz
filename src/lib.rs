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

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for i in 0..width {
        for j in 0..height {
            //data.push((((x / 4.0)/x)*255.0) as u8);
            data.push(255);
            data.push(255);
            data.push(255);
            data.push(255);
        }
    }

    data
}

