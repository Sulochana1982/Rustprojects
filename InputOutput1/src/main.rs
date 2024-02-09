

extern crate InputOutput1;
use InputOutput1::Config;
use std::env;
use std::process;



// cargo run > output.txt
fn main()
{
    let args: Vec<String> = env::args().collect();
    
    let  config : Config = Config::new(&args).unwrap_or_else(|err| {
           println!("Problem parsing arguments :{}", err);
            process::exit(1);
         });
                                                  

    println!("searching: {}", config.query);
    println!("In file {}", config.filename);






    if let Err(e) = InputOutput1::run(config)
   {
    println!("Application Error: {}", e);
   process::exit(1);
   }
}