use std::env;
use std::fs::File;
use std::io::Read;

struct Config{
    query: String,
    filename: String,
}



fn run(config: Config) 
{
    println!("searching: {}", config.query);
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong");
    println!("contents of the file: {}", contents);
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

