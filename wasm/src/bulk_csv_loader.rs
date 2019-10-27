use super::point_dataset::{PointDataset, PointWithData};
use geo::{Geometry, Point};
use rstar::{RTree, RTreeObject, AABB};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
struct BulkCSVLoader {
  latitudes: Vec<f32>,
  longitudes: Vec<f32>,
  lng_min: f32,
  lat_min: f32,
  lng_max: f32,
  lat_max: f32,
  no_rows: u32,
  values: HashMap<String, Vec<f32>>,
}

#[wasm_bindgen]
impl BulkCSVLoader {
  pub fn new() -> Self {
    BulkCSVLoader {
      latitudes: Vec::new(),
      longitudes: Vec::new(),
      lng_min: 180.0,
      lat_min: 90.0,
      lng_max: -180.0,
      lat_max: -90.0,
      no_rows: 0,
      values: HashMap::new(),
    }
  }

  pub fn append_csv(&mut self, csv: String, lat_col: u8, lng_col: u8, data_columns: JsValue) {
    // console::log_2(&"chunk is ".into(), &csv.into());
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let data_cols: HashMap<String, u32> = match serde_wasm_bindgen::from_value(data_columns) {
      Ok(val) => val,
      Err(err) => {
        console::log_2(&"Issue reading the hash map".into(), &err.into());
        HashMap::new()
      }
    };

    for record in reader.records() {
      match record {
        Ok(line) => {
          for key in data_cols.keys() {
            let row_no = data_cols.get(key).unwrap();

            let val = match line[*row_no as usize].parse() {
              Ok(v) => v,
              Err(_) => 0.0,
            };

            self
              .values
              .entry(key.to_string())
              .or_insert(Vec::new())
              .push(val);
          }

          let lat: f32 = match line[lat_col as usize].parse() {
            Ok(val) => val,
            Err(e) => {
              console::log_2(&"failed to parse lat col".into(), &e.to_string().into());
              -9999.0
            }
          };

          let lng: f32 = match line[lng_col as usize].parse() {
            Ok(val) => val,
            Err(_) => {
              console::log_1(&"failed to parse lng col".into());
              -99999.0
            }
          };

          if lng > -180.0
            && lat > -90.0
            && !approx_eq!(f32, lat, 0.0, ulps = 3)
            && !approx_eq!(f32, lat, 0.0, ulps = 3)
          {
            if lng < self.lng_min {
              self.lng_min = lng;
            };
            if lng > self.lng_max {
              self.lng_max = lng;
            };
            if lat < self.lat_min {
              self.lat_min = lat;
            };
            if lat > self.lat_max {
              self.lat_max = lat;
            };
          };

          self.latitudes.push(lat);
          self.longitudes.push(lng);

          self.no_rows = self.no_rows + 1;
        }
        Err(_) => console::log_1(&"Failed to parse row".into()),
      };
    }
  }

  pub fn create_dataset(self) -> PointDataset {
    let mut points_with_data: Vec<PointWithData> = vec![];

    for i in 0..self.latitudes.len() {
      let mut data: HashMap<String, f32> = HashMap::new();
      for key in self.values.keys() {
        let val = self.values.get(&key.to_string()).unwrap()[i];
        data.insert(key.to_string(), val);
      }
      points_with_data.push(PointWithData {
        coords: Point::new(self.longitudes[i], self.latitudes[i]),
        data: data,
      })
    }
    PointDataset::new_from_loader(
      RTree::bulk_load(points_with_data),
      self.no_rows,
      self.lng_min,
      self.lat_min,
      self.lng_max,
      self.lat_max,
    )
  }
}
