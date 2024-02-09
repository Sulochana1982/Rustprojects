



pub struct Ohlc{
    pub open : f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
 }
 
 #[derive(Debug)]
 pub struct Camarilla {
     pub  r_4: f32,
     pub r_3: f32,
     pub r_2: f32,
     pub r_1: f32,
     pub p_p: f32,
     pub s_1: f32,
     pub s_2: f32,
     pub s_3: f32,
     pub s_4: f32,
 }
 
 
 pub fn camarilla_pivots(d : Ohlc) -> Camarilla {
     Camarilla  {
         r_4: d.close + (d.high - d.low)*1.1/2.0,
         r_3: d.close + (d.high - d.low)*1.1/4.0,
         r_2: d.close + (d.high - d.low)*1.1/6.0,
         r_1: d.close + (d.high - d.low)*1.1/12.0,
         p_p : (d.high+d.low+d.close)/3.0,
         s_1: d.close - (d.high - d.low)*1.1/12.0,
         s_2: d.close - (d.high - d.low)*1.1/6.0,
         s_3: d.close - (d.high - d.low)*1.1/4.0,
         s_4: d.close - (d.high - d.low)*1.1/2.0,
      }
     }