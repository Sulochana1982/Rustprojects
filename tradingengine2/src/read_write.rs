
extern crate csv;
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;
use std::io::BufReader;

use super::camarilla_pivot::Ohlc;



pub fn read_file()
{

    let file = File::open("ohlc.csv").expect("Could not open the CSV file");

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));


    for result in rdr.records() {
        let record: StringRecord = result.expect("error reading CSV record");

        let mut arr:[f32; 4]= [0.0, 0.0, 0.0 ,0.0];

        arr[0]= record[0].trim().parse().expect("Failed to parse open");
        arr[1]=record[1].trim().parse().expect("Failed to parse high");
        arr[2] = record[2].trim().parse().expect("Failed to parse low");
        arr[3] = record[3].trim().parse().expect("Failed to parse close");

        Ohlc::new(arr);
    
        

}

}






 
  

