#![feature(async_await)]

extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate float_cmp;

use wasm_bindgen::prelude::*;
use web_sys::console;

mod bulk_csv_loader;
mod point_dataset;
mod polygon_dataset;
mod ramp;
mod shader;

use point_dataset::PointDataset;
use polygon_dataset::PolygonDataset;

use std::collections::HashMap;

#[wasm_bindgen]
pub fn load_geojson(geojson: String) -> PolygonDataset {
  console::log_1(&"parsing geojson".into());
  PolygonDataset::from_geojson(geojson)
}

#[wasm_bindgen]
pub fn agg_in_poly(polys: &PolygonDataset, points: &PointDataset) -> JsValue {
  let mut idCounts: HashMap<u32, HashMap<String, f32>> = HashMap::new();

  for (i, poly) in polys.polys().iter().enumerate() {
    let id = polys.geom_id(i as usize);
    let aggregates = points.agg_in(poly);
    idCounts.insert(id, aggregates);
  }
  let result = CountResult { counts: idCounts };
  JsValue::from_serde(&result).unwrap()
}

#[derive(Serialize)]
pub struct CountResult {
  pub counts: HashMap<u32, HashMap<String, f32>>,
}
