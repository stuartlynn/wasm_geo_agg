extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use web_sys::console;

mod dataset;
mod polygon_dataset;
mod ramp;

use dataset::Dataset;
use polygon_dataset::PolygonDataset;

use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;


#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s:&str);
}

#[no_mangle]
pub fn add(x:f32, y:f32)->f32{
  x+y
}

#[wasm_bindgen]
pub fn load_csv(csv:String)->Dataset{
  Dataset::from_csv_str(csv)
}

#[wasm_bindgen]
pub fn load_geojson(geojson:String)->PolygonDataset{
  console::log_1(&"parsing geojson".into());
  PolygonDataset::from_geojson(geojson)
}

#[wasm_bindgen]
pub fn count_in_poly(polys:&PolygonDataset, points:&Dataset)->JsValue{
  let mut idCounts: HashMap<String, u32> = HashMap::<String, u32>::new();
 
  for (i,poly) in polys.polys().iter().enumerate() {
     let id = polys.geom_id(i as usize);
     let count = points.count_in(poly);
     idCounts.insert(id, count);
  };
  let result = CountResult{counts:idCounts};
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn parse_csv(csv: String)->JsValue{
  let dataset = Dataset::from_csv_str(csv);
  let count_js: JsValue = dataset.no_rows.into();
  dataset.agg_to_blocks();
  return count_js;
}

#[wasm_bindgen]
pub fn test_map(hold: JsValue, map: JsValue){
  let hash_map: HashMap<String,f32> = match serde_wasm_bindgen::from_value(map){
    Ok(val)=>val,
    Err(_)=>{
       console::log_1(&"issue reading map in test".into());
       HashMap::new() 
    }
  };

  match hash_map.get("test"){
    Some(val)=> {
      let string_thing:String = format!("{}",val);
      console::log_2(&"got val ".into(), &string_thing.into());
    },
    None => console::log_1(&"No key in here".into())
  }

}

#[derive(Serialize)]
pub struct CountResult{
  pub counts: HashMap<String,u32>
}

