extern crate csv;
extern crate rstar;

use geo::{Geometry, Point};
use rstar::{RTree, RTreeObject, AABB};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use web_sys::console;

use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;

pub struct PointWithData {
  pub coords: Point<f32>,
  pub data: HashMap<String, f32>,
}

impl RTreeObject for PointWithData {
  type Envelope = AABB<[f32; 2]>;

  fn envelope(&self) -> Self::Envelope {
    AABB::from_point([self.coords.x(), self.coords.y()])
  }
}

#[wasm_bindgen]
pub struct PointDataset {
  rtree: RTree<PointWithData>,
  pub no_rows: u32,
  // coords: Vec<PointWithData>,
  pub lat_max: f32,
  pub lat_min: f32,
  pub lng_max: f32,
  pub lng_min: f32,
}

#[wasm_bindgen]
impl PointDataset {
  // pub fn new_empty() -> PointDataset {
  //   PointDataset {
  //     rtree: RTree::new(),
  //     no_rows: 0,
  //     // coords: Vec::new(),
  //     lng_min: 180.0,
  //     lat_min: 90.0,
  //     lng_max: -180.0,
  //     lat_max: -90.0,
  //   }
  // }

  // pub fn generateTree(mut self) {
  //   let tree = RTree::bulk_load(self.coords);
  //   self.rtree = tree;
  // }

  //   pub fn from_csv(csv: String, lat_col: u8, lng_col: u8) -> PointDataset {
  //     let mut columns: HashMap<String, Vec<f32>> = HashMap::new();
  //     let mut coords: Vec<Point<f32>> = Vec::new();

  //     console::log_1(&"Starting rust parse".into());
  //     console::log_3(&"lat col is ".into(), &lat_col.into(), &lng_col.into());

  //     let mut reader = csv::Reader::from_reader(csv.as_bytes());
  //     let mut count = 0;
  //     for record in reader.records() {
  //       match record {
  //         Ok(line) => {
  //           count = count + 1;

  //           let lat: f32 = match line[lat_col as usize].parse() {
  //             Ok(val) => val,
  //             Err(e) => {
  //               console::log_1(&"failed to parse lat col".into());
  //               0.0
  //             }
  //           };

  //           let lng: f32 = match line[lng_col as usize].parse() {
  //             Ok(val) => val,
  //             Err(e) => {
  //               console::log_1(&"failed to parse lng col".into());
  //               0.0
  //             }
  //           };
  //           coords.push(Point::new(lng, lat));
  //         }
  //         Err(_) => console::log_1(&"Failed to parse row".into()),
  //       };
  //     }
  //     console::log_1(&"Done parse".into());

  //     return PointDataset::new(count, coords);
  //   }
}

impl PointDataset {
  pub fn new(no_rows: u32) -> Self {
    console::log_1(&"cerateing new point dataset".into());
    let tree = RTree::new();
    PointDataset {
      rtree: tree,
      no_rows: no_rows,
      // coords: Vec::new(),
      lng_min: -180.0,
      lat_min: -90.0,
      lng_max: 180.0,
      lat_max: 90.0,
    }
  }
  pub fn new_from_loader(
    tree: RTree<PointWithData>,
    no_rows: u32,
    lng_min: f32,
    lat_min: f32,
    lng_max: f32,
    lat_max: f32,
  ) -> Self {
    PointDataset {
      rtree: tree,
      no_rows: no_rows,
      lng_min: lng_min,
      lat_min: lat_min,
      lng_max: lng_max,
      lat_max: lat_max,
    }
  }

  pub fn agg_in(&self, poly: &Geometry<f32>) -> HashMap<String, f32> {
    let bounds = match poly {
      Geometry::Polygon(p) => Ok(p.bounding_rect().unwrap()),
      Geometry::MultiPolygon(p) => Ok(p.bounding_rect().unwrap()),
      _ => Err("Wrong poly type"),
    }
    .unwrap();

    let target_bounds =
      AABB::from_corners([bounds.min.x, bounds.min.y], [bounds.max.x, bounds.max.y]);
    let mut results: HashMap<String, f32> = HashMap::new();

    let candidates = self.rtree.locate_in_envelope(&target_bounds);
    let hits = candidates.filter(|p| match poly {
      Geometry::MultiPolygon(target) => target.contains(&p.coords),
      _ => false,
    });

    let mut count = 0;

    for hit in hits {
      count = count + 1;
      for key in hit.data.keys() {
        let val = hit.data.get(key).unwrap();
        *results.entry(key.to_string()).or_insert(0.0) += val;
      }
    }

    results.insert("count".to_string(), count as f32);

    results
  }

  pub fn coords(&self) -> Vec<Point<f32>> {
    self
      .rtree
      .iter()
      .map(|p| Point::new(p.coords.x(), p.coords.y()))
      .collect()
  }
}
