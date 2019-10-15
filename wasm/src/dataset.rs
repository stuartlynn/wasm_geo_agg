extern crate csv;
extern crate rstar;

use web_sys::{console, ImageData};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use geo::{Point,Geometry};
use rstar::{RTree,AABB};

use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;

use super::ramp::ramp;

#[wasm_bindgen]
pub struct Dataset{
  rtree: RTree<[f32; 2]>,
  pub no_rows: u32,
  coords: Vec<Point<f32>>,
  block_ids : Vec<u32>,
  diameter: Vec<f32>,
}

#[wasm_bindgen]
impl Dataset{
  

  pub fn render_to_canvas(&self, lng_start: f32, lat_start: f32, lng_end: f32, lat_end: f32, res: u32, log: bool, pixel_density: i8){
     console::log_1(&"Rendering to canvas".into());
     let pixelDensity = self.pixel_density(lng_start,lat_start,lng_end,lat_end, res as usize, pixel_density);
     let mut data  = ramp(pixelDensity,log,true);

     console::log_2(&"size of result is ".into(), &(data.len() as u32).into());
     let document = web_sys::window().unwrap().document().unwrap();
     let canvas = document.get_element_by_id("canvas").unwrap();
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
     let data  = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), res as u32,res as u32).unwrap();

     context.put_image_data(&data,0.0,0.0);
  }
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

impl Dataset{

  pub fn new(no_rows: u32, coords:Vec<Point<f32>>, block_ids:Vec<u32>,diameter:Vec<f32>)->Self{
    let tree = RTree::bulk_load(coords.clone().iter().map(|p| [p.x(),p.y()]).collect());
    Dataset{
      rtree:tree,
      no_rows:no_rows,
      coords:coords,
      block_ids:block_ids,
      diameter:diameter
    }
  }

  pub fn agg_to_blocks(&self)->f32{
    let unique_blocks : HashSet<u32> = self.block_ids.clone().drain(..).collect();
    let no_blocks = unique_blocks.len() as u32;
    console::log_2(&"No unique blocks ".into(), &no_blocks.into());
    return 2.0;
  }

  pub fn count_in(&self, poly: &Geometry<f32>)->u32{
    
    let bounds = match poly{
      Geometry::Polygon(p)=> Ok(p.bounding_rect().unwrap()),
      Geometry::MultiPolygon(p)=> Ok(p.bounding_rect().unwrap()),
      _=> Err("Wrong poly type")
    }.unwrap();

    let target_bounds = AABB::from_corners([bounds.min.x, bounds.min.y], [bounds.max.x,bounds.max.y]);
    let candidates = self.rtree.locate_in_envelope(&target_bounds);
    let hits = candidates.filter(|p| match poly {
        Geometry::MultiPolygon(target)=>target.contains(&Point::new(p[0],p[1])),
        _ => false
    });
    hits.count() as u32
  }

  pub fn coords(&self) -> &Vec<Point<f32>>{
    &self.coords
  }

  

  fn pixel_density(&self, lng_start:f32, lat_start: f32, lng_end: f32, lat_end: f32, res:usize, pixel_radius: i8) -> Vec<f32> {
      let mut counts: Vec<f32> = vec![0.0;res*res];
      let lat_bin_size = (lat_end-lat_start)/(res as f32);
      let lng_bin_size = (lng_end-lng_start)/(res as f32);

      console::log_2(&"lat bin size is ".into(), &lat_bin_size.into());
      console::log_2(&"lng bin size is ".into(), &lng_bin_size.into());

      for point in self.coords.iter(){
         let lat = point.y();
         let lng = point.x();
         if lat > lat_start && lat < lat_end  && lng > lng_start && lng < lng_end{
             let lat_bin = (res as u32) - ((lat -lat_start) / lat_bin_size) as u32;
             let lng_bin = ((lng -lng_start) / lng_bin_size) as u32;
             for offset_x in -pixel_radius..pixel_radius{
                 for offset_y in -pixel_radius..pixel_radius{
                     let lat_bin_offset = lat_bin + (offset_y as u32);
                     let lng_bin_offset = lng_bin + (offset_x as u32);
                     if lat_bin_offset >= 0 && lat_bin_offset < (res as u32) -1  && lng_bin_offset>=0 && lng_bin_offset < (res as u32)-1{
                         let index = (lng_bin_offset + lat_bin_offset*(res as u32)) as usize;
                         counts[index] = counts[index] + 1.0;
                     }
                }
            }
         }
      }
      counts
  }

  pub fn from_csv_str(csv: String)->Self{
    let mut points: Vec<Point<f32>> = Vec::new();
    // let mut longitudes: Vec<f32> = Vec::new();
    let mut block_ids: Vec<u32> =  Vec::new();
    let mut diameter: Vec<f32> =  Vec::new();
    console::log_1(&"Starting rust parse".into());
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut count = 0;
    for record in reader.records(){
       let record  = record.unwrap();
       count = count + 1;

       let lat:f32  = record[38].parse().unwrap();
       let lng:f32  = record[39].parse().unwrap();
       let block:u32 = record[2].parse().unwrap();
       let diam:f32 = record[4].parse().unwrap();
       points.push(Point::new(lng,lat));
       block_ids.push(block);
       diameter.push(diam);
    }
    console::log_1(&"Done rust parse".into());
    return Dataset::new(
      count,
      points,
      block_ids,
      diameter
    )

  }
}
