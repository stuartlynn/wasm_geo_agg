use super::point_dataset::PointDataset;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{console, ImageData};

use super::ramp::ramp;

#[wasm_bindgen]
pub fn render_points_to_canvas(
    canvasId: String,
    dataset: &PointDataset,
    lng_start: f32,
    lat_start: f32,
    lng_end: f32,
    lat_end: f32,
    res: u32,
    log: bool,
    pixels_per_point: i8,
) {
    console::log_1(&"Rendering to canvas".into());
    let density = pixel_density(
        dataset,
        lng_start,
        lat_start,
        lng_end,
        lat_end,
        res as usize,
        pixels_per_point,
    );
    let mut data = ramp(density, log, true);

    console::log_2(&"size of result is ".into(), &(data.len() as u32).into());
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(&canvasId).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let width = canvas.width();
    let height = canvas.height();

    console::log_3(&"canvas size is ".into(), &width.into(), &height.into());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), res as u32, res as u32)
            .unwrap();

    context.put_image_data(&data, 0.0, 0.0);
}

fn pixel_density(
    dataset: &PointDataset,
    lng_start: f32,
    lat_start: f32,
    lng_end: f32,
    lat_end: f32,
    res: usize,
    pixel_radius: i8,
) -> Vec<f32> {
    let mut counts: Vec<f32> = vec![0.0; res * res];
    let lat_bin_size = (lat_end - lat_start) / (res as f32);
    let lng_bin_size = (lng_end - lng_start) / (res as f32);

    console::log_2(&"lat bin size is ".into(), &lat_bin_size.into());
    console::log_2(&"lng bin size is ".into(), &lng_bin_size.into());

    for point in dataset.coords().iter() {
        let lat = point.y();
        let lng = point.x();
        if lat > lat_start && lat < lat_end && lng > lng_start && lng < lng_end {
            let lat_bin = (res as u32) - ((lat - lat_start) / lat_bin_size) as u32;
            let lng_bin = ((lng - lng_start) / lng_bin_size) as u32;
            for offset_x in -pixel_radius..pixel_radius {
                for offset_y in -pixel_radius..pixel_radius {
                    let lat_bin_offset = lat_bin + (offset_y as u32);
                    let lng_bin_offset = lng_bin + (offset_x as u32);
                    if lat_bin_offset >= 0
                        && lat_bin_offset < (res as u32) - 1
                        && lng_bin_offset >= 0
                        && lng_bin_offset < (res as u32) - 1
                    {
                        let index = (lng_bin_offset + lat_bin_offset * (res as u32)) as usize;
                        counts[index] = counts[index] + 1.0;
                    }
                }
            }
        }
    }
    counts
}
