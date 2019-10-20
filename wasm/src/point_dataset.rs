extern crate csv;
extern crate rstar;

use geo::{Geometry, Point};
use rstar::{RTree, AABB};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

use web_sys::console;

use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;

#[wasm_bindgen]
pub struct PointDataset {
  rtree: Option<RTree<[f32; 2]>>,
  pub no_rows: u32,
  coords: Vec<Point<f32>>,
  ids: Vec<u32>,
}

// ![derive(Copy,Clone,PartialEq,Debug)]
// struct PointWithId {
//   coord:Point<f32>,
//   string:id
// }

// impl Point for PointWithId{

//   const DIMENSIONS: usize = 2;

//   fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self
//     {
//       IntegerPoint {
//         : generator(0),
//         y: generator(1),

//       }
//     }
// }

#[wasm_bindgen]
impl PointDataset {
  pub fn new_empty() -> PointDataset {
    PointDataset {
      rtree: Option::None,
      no_rows: 0,
      coords: Vec::new(),
      ids: Vec::new(),
    }
  }

  pub fn generateTree(&mut self) {
    let tree = RTree::bulk_load(self.coords.clone().iter().map(|p| [p.x(), p.y()]).collect());
    self.rtree = Some(tree);
    console::log_1(&"Created rtree".into());
  }

  pub fn append_csv(&mut self, csv: String, lat_col: u8, lng_col: u8) {
    // console::log_2(&"chunk is ".into(), &csv.into());
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
      match record {
        Ok(line) => {
          let lat: f32 = match line[lat_col as usize].parse() {
            Ok(val) => val,
            Err(e) => {
              console::log_2(&"failed to parse lat col".into(), &e.to_string().into());
              0.0
            }
          };

          let lng: f32 = match line[lng_col as usize].parse() {
            Ok(val) => val,
            Err(e) => {
              console::log_1(&"failed to parse lng col".into());
              0.0
            }
          };
          self.coords.push(Point::new(lng, lat));
          self.ids.push(self.no_rows);
          self.no_rows = self.no_rows + 1
        }
        Err(_) => console::log_1(&"Failed to parse row".into()),
      };
    }
  }

  pub fn from_csv(csv: String, lat_col: u8, lng_col: u8) -> PointDataset {
    // let mut longitudes: Vec<f32> = Vec::new();
    let mut ids: Vec<u32> = Vec::new();
    let mut columns: HashMap<String, Vec<f32>> = HashMap::new();
    let mut coords: Vec<Point<f32>> = Vec::new();

    console::log_1(&"Starting rust parse".into());
    console::log_3(&"lat col is ".into(), &lat_col.into(), &lng_col.into());

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut count = 0;
    for record in reader.records() {
      match record {
        Ok(line) => {
          count = count + 1;
          let lat: f32 = match line[lat_col as usize].parse() {
            Ok(val) => val,
            Err(e) => {
              console::log_1(&"failed to parse lat col".into());
              0.0
            }
          };

          let lng: f32 = match line[lng_col as usize].parse() {
            Ok(val) => val,
            Err(e) => {
              console::log_1(&"failed to parse lng col".into());
              0.0
            }
          };
          coords.push(Point::new(lng, lat));
          ids.push(count);
        }
        Err(_) => console::log_1(&"Failed to parse row".into()),
      };
    }
    console::log_1(&"Done parse".into());

    return PointDataset::new(count, coords, ids);
  }
}

impl PointDataset {
  pub fn new(no_rows: u32, coords: Vec<Point<f32>>, ids: Vec<u32>) -> Self {
    console::log_1(&"cerateing new point dataset".into());
    let tree = RTree::bulk_load(coords.clone().iter().map(|p| [p.x(), p.y()]).collect());
    PointDataset {
      rtree: Some(tree),
      no_rows: no_rows,
      coords: coords,
      ids: ids,
    }
  }

  pub fn agg_to_blocks(&self) -> f32 {
    let unique_blocks: HashSet<u32> = self.ids.clone().drain(..).collect();
    let no_blocks = unique_blocks.len() as u32;
    console::log_2(&"No unique blocks ".into(), &no_blocks.into());
    return 2.0;
  }

  pub fn count_in(&self, poly: &Geometry<f32>) -> u32 {
    let bounds = match poly {
      Geometry::Polygon(p) => Ok(p.bounding_rect().unwrap()),
      Geometry::MultiPolygon(p) => Ok(p.bounding_rect().unwrap()),
      _ => Err("Wrong poly type"),
    }
    .unwrap();

    let target_bounds =
      AABB::from_corners([bounds.min.x, bounds.min.y], [bounds.max.x, bounds.max.y]);
    let count = match (&self.rtree) {
      Some(tree) => {
        let candidates = tree.locate_in_envelope(&target_bounds);
        let hits = candidates.filter(|p| match poly {
          Geometry::MultiPolygon(target) => target.contains(&Point::new(p[0], p[1])),
          _ => false,
        });
        hits.count() as u32
      }
      None => 0,
    };
    count
  }

  pub fn coords(&self) -> &Vec<Point<f32>> {
    &self.coords
  }
}
