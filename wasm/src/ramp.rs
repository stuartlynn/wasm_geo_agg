pub fn ramp(agg: Vec<f32>, log: bool, rgb:bool)-> Vec<u8>{
     let mut max = -std::f32::INFINITY;
     let mut min = std::f32::INFINITY;
     let mut result = Vec::new();

     let mut min = 0.0;
     for val in agg.iter(){
        if *val > max{
          max = *val;
        }
        if *val  < min{
          min = *val
        }
     }

     if log{
        if max > 0.0{
          max = max.log10();
        }
        else{
          max = 0.0;
        }
        if min >0.0{
          min = min.log10();
        }
        else{
          min = 0.0;
        }
     }

     let mut count_non_zero  = 0;

     for val in agg.into_iter(){
       if val > 0.0 {
          count_non_zero = count_non_zero + 1;
       }

       let mut mval = val;
       if log {
         if mval > 0.0 {
           mval = mval.log10()
         }
         else{
           mval=0.0
         }
       }
       let r_val = (mval - min)*255.0/(max-min);
       //let r_val = 0 as u8;

       result.push(r_val as u8);
       if rgb {
            result.push(r_val as u8);
            result.push(r_val as u8);
            result.push(255 as u8);
       }
     }
     result
  }