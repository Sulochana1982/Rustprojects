use std::env;
use std::fs::File;
use std::io::Read;
use std::error::Error;

struct Config{
    query: String,
    filename: String,
}



fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    println!("searching: {}", config.query);
    
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    println!("contents of the file: {}", contents);
    Ok(())
}


    




fn main()
{
    let args: Vec<String> = env::args().collect();
    let config: Config = Config{
        query: args[1].clone(),
        filename: args[2].clone(),
    };
         
    run(config);

   

}  

