
extern crate csv;
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;
use std::io::BufReader;

use super::camarilla_pivot::Ohlc;
use super::camarilla_pivot::Camarilla;
use super::camarilla_pivot::camarilla_pivots;


pub fn read_file()
{

    let file = File::open("ohlc.csv").expect("Could not open the CSV file");

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));


    for result in rdr.records() {
        let record: StringRecord = result.expect("error reading CSV record");
    
        let data = Ohlc {
          open: record[0].trim().parse().expect("Failed to parse open"),
          high: record[1].trim().parse().expect("Failed to parse high"),
          low: record[2].trim().parse().expect("Failed to parse low"),
          close: record[3].trim().parse().expect("Failed to parse close"),
         } ;
        
         
          println!("Open : {}, High: {}, Low: {}, Close: {}", data.open, data.high, data.low, data.close);
          
          write_file(data);

}

}


pub fn write_file(data: Ohlc)
{
    
          let pivotpoints:Camarilla = camarilla_pivots(data);
          println!("Pivot Points: PP:{}, r_1:{}, r_2:{}, r_3:{}, r_4:{}, s_1:{}, s_2:{}, s_3:{}, s_4:{}", pivotpoints.p_p, pivotpoints.r_1, pivotpoints.r_2,
        pivotpoints.r_3, pivotpoints.r_4, pivotpoints.s_1, pivotpoints.s_2, pivotpoints.s_3, pivotpoints.s_4);

                

 }



 
  

