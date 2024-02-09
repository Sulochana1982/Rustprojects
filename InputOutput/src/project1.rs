use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let args: Vec<String> = env::args().collect();
    
    let (query, filename) : (&str, &str) = parse_config(&args);

    println!("searching: {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong");
    println!("contents of the file: {}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str)
{
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}