
// use std::fs::File;

// use std::error::Error;
// use std::io::prelude::*;


// pub fn run(config: Config) -> Result<(), Box<dyn Error>>
// {
//     println!("searching: {}", config.query);
//     let mut f = File::open(config.filename)?;
//     let mut contents = String::new();

//     for line in search(&config.query, &contents){
//         println!("{}", line);
//     }

//     // f.read_to_string(&mut contents)?;
//     // println!("contents of the file: {}", contents);
//     Ok(())
// }

// fn search<'a> (query: &str, contents: & 'a str) -> Vec<& 'a str>
// {
//     let mut results = Vec::new();
//     for line in contents.lines()
//     {
//         if line.contains(query)
//         {
//             results.push(line);
//         }
//     }
//     results
// }


// pub struct Config{
//     pub query: String,
//     pub filename: String,
// }
// impl Config{


// pub fn new(args: &[String]) -> Result<Config, &'static str>
// {

//     if args.len()<3{
//         return Err("not enough arguments");
//     }
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Ok(Config{query, filename })
// }
// }