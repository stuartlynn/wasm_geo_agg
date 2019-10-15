extern crate geo;
extern crate geojson;
extern crate geo_types;
extern crate serde_wasm_bindgen;


use std::convert::TryInto;
use geo_types::{Polygon,MultiPolygon,Geometry,Point};


use geo::prelude::*;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::area::Area;
use geo::algorithm::contains::Contains;
use std::collections::HashMap;


use geojson::{GeoJson,Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;

use web_sys::{console};

use super::ramp::ramp;



#[wasm_bindgen]
pub struct PolygonDataset{
   pub no_objects: u32,
   objects: Vec<Geometry<f32>>,
   bounds: [f32;4],
   ids:Vec<String>
}

#[wasm_bindgen]
impl PolygonDataset{
  
  pub fn from_geojson(geojson_str:String)->Self{
     let geojson = geojson_str.parse::<GeoJson>().unwrap();
     let (geoms,ids) = PolygonDataset::process_geojson(&geojson);
     PolygonDataset{
       no_objects: 0,
       objects: geoms,
       bounds: [0.0,0.0,0.0,0.0],
       ids: ids
     }
  } 

  pub fn display_with_counts(&self,
             counts: JsValue,
             lng_start: f32, 
             lat_start: f32, 
             lng_end: f32, 
             lat_end: f32
             ){

     let count_map: HashMap<String,f32> = match serde_wasm_bindgen::from_value(counts){
         Ok(val)=> val,
         Err(err)=> {
             console::log_2(&"Issue reading the hash map".into(), &err.into());
             HashMap::new()
         }
     };

     let document = web_sys::window().unwrap().document().unwrap();
     let canvas = document.get_element_by_id("canvas2").unwrap();
     let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

     let width = canvas.width();
     let height = canvas.height();

     let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    context.set_fill_style(&"black".into());
    context.clear_rect(0.0,0.0,width as f64,height as f64);
    context.fill_rect(0.0,0.0,width as f64,height as f64);

    let mut count_vals: Vec<f32> = Vec::new();
    
    for index in  0..self.objects.len(){
        let id  = &self.ids[index];
        let count  = match count_map.get(id){
            Some(val) => *val,
            None => 0.0
        };
        count_vals.push(count);
    }
    
    let ramped_vals = ramp(count_vals,true,false);


    for index in  0..self.objects.len(){
        let object = &self.objects[index];
        match object {
            Geometry::MultiPolygon(poly)=>{
                let first = &poly.0[0];
                let exterior  = first.exterior().clone();

                let col = ramped_vals[index];
                let col_str = format!("rgba({},{},{}", col,col,col);
                context.set_fill_style(&col_str.into());
                context.set_stroke_style(&"blue".into());

                context.begin_path();
                context.move_to(75.0, 50.0);
                let mut start = true;

                for coord in exterior{
                    let x = (coord.x - lng_start)*(width as f32)/(lng_end-lng_start);
                    let y = (height as f32) - (coord.y - lat_start)*(width as f32)/(lat_end-lat_start);
                    if start {
                        context.move_to( x as f64 ,  y as f64);
                        start=false;
                    }
                    else{
                        context.line_to(x as f64, y as f64);
                    };

                }
                context.fill();
            },
            _ => println!("issue")
        }
        
    }
   }

  pub fn geom_id(&self,index:usize)-> String{
     return self.ids[index].clone();
  }

  pub fn geom_area(&self,index: usize) -> f32{
     match(&self.objects[index]){
         Geometry::Polygon(poly)=> {
             console::log_2(&"MATCHING AS POLYGON".into(),&poly.area().into());
             poly.area()
         }
         Geometry::MultiPolygon(poly)=>{
             let area  = poly.area();
             console::log_2(&"MATCHING AS MULTIPOLY".into(), &area.into());
             let bbox = poly.bounding_rect().unwrap();
             console::log_5(&"MATCHING AS MULTIPOLY".into(),     
                            &bbox.min.x.into(),
                            &bbox.min.y.into(),
                            &bbox.max.x.into(),
                            &bbox.max.y.into());

             area
         },
         _=> {
             console::log_1(&"MATCHING OTHER".into());
             0.0
         }
     }
  }
}

impl PolygonDataset{

   pub fn polys(&self)->&Vec<Geometry<f32>>{
     return &self.objects;   
   }

   

   fn process_geojson(geojson: &GeoJson)->(Vec<Geometry<f32>>, Vec<String>){
        let mut no_polygons = 0;
        let mut no_multi_polygons =0;
   
        let  mut geometries: Vec<Geometry<f32>> = Vec::new();
        let mut ids: Vec<String> = Vec::new();

        match *geojson {
            GeoJson::FeatureCollection(ref ctn)=> for feature in &ctn.features{
                let vals = feature.properties.clone().unwrap();
                let id = vals.get("bctcb2010").unwrap();
                ids.push(id.to_string());

                if let Some(ref geom) = feature.geometry{
                    match &geom.value {
                       Value::Polygon(poly) => {
                           let p : Polygon<f32> = geom.value.clone().try_into().unwrap();
                           geometries.push(p.into());
                           no_polygons = no_polygons +1;
                       },
                       Value::MultiPolygon(_) =>{ 
                           let p : MultiPolygon<f32> = geom.value.clone().try_into().unwrap();
                           geometries.push(p.into());
                           no_multi_polygons =no_multi_polygons+1;
                       },
                       _ => println!("matched something else")
                    }
                }  
            },
            _ => println!("matched not geojson")
        };
        console::log_2(&"No polygons ".into(), &no_polygons.into());
        console::log_2(&"No multi polygons ".into(), &no_multi_polygons.into());
        (geometries,ids)
 }
}
