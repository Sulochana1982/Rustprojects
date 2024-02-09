use std::env;
use std::fs::File;
use std::io::Read;
struct Config{
    query: String,
    filename: String,
}

impl Config{


fn new(args: &[String]) -> Config
{
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{

        query,
        filename,
    }
}
}


fn main()
{
    let args: Vec<String> = env::args().collect();
    
    let  config : Config = Config::new(&args);

    println!("searching: {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong");
    println!("contents of the file: {}", contents);
}
