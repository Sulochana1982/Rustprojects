
use std::fs::File;
use std::io::ErrorKind;
use std::fs;

use super::camarilla_pivot::Camarilla;

pub fn write_file(pivotpoints: Camarilla)
{
        
          println!("Pivot Points: PP:{}, r_1:{}, r_2:{}, r_3:{}, r_4:{}, s_1:{}, s_2:{}, s_3:{}, s_4:{}", pivotpoints.p_p, pivotpoints.r_1, pivotpoints.r_2,
        pivotpoints.r_3, pivotpoints.r_4, pivotpoints.s_1, pivotpoints.s_2, pivotpoints.s_3, pivotpoints.s_4);


       let s1: String = format!("{} {} {} {} {} {} {} {} {}", pivotpoints.p_p, pivotpoints.r_1, pivotpoints.r_2,
        pivotpoints.r_3, pivotpoints.r_4, pivotpoints.s_1, pivotpoints.s_2, pivotpoints.s_3, pivotpoints.s_4); 
        println!();
        println!();
        println!("{}", s1);
        println!();

let f = File::open("pivots.csv");
let f = match f {
  Ok(file) => file,
                          
  Err(ref error) if error.kind() == ErrorKind::NotFound => {
   match File::create("pivots.csv"){
     Ok(fc) => fc,
     Err(e) => {
       panic!( "Tried to create file but there was a problem: {:?}", e)
     },
   }
  },

  Err(error) => {
   panic!(
     "There was a problem opening the file: {:?}", error
   )
  },
 };

   fs::write("pivots.csv", s1). expect("Unable to write file");





}
       

       

        
        
        

                

 