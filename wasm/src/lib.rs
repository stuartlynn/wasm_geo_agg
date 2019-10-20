#![feature(async_await)]

extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub fn count_in_poly(polys: &PolygonDataset, points: &PointDataset) -> JsValue {
  let mut idCounts: HashMap<u32, u32> = HashMap::new();

  for (i, poly) in polys.polys().iter().enumerate() {
    let id = polys.geom_id(i as usize);
    let count = points.count_in(poly);
    idCounts.insert(id, count);
  }
  let result = CountResult { counts: idCounts };
  JsValue::from_serde(&result).unwrap()
}

// #[wasm_bindgen]
// pub fn parse_csv(csv: String) -> JsValue {
//   let dataset = PointDataset::from_csv_str(csv);
//   let count_js: JsValue = dataset.no_rows.into();
//   dataset.agg_to_blocks();
//   return count_js;
// }

#[derive(Serialize)]
pub struct CountResult {
  pub counts: HashMap<u32, u32>,
}
