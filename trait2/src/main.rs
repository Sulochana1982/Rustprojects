//extern crate trait1;
use trait2::Summarizable;

struct Whether1{
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation:f64,
}

impl Summarizable for Whether1{
    fn summary(&self)-> String{
        format!("{},  {},  {}", self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
    // fn add()
    // {
    //     println!("{}", 100+200);
    // }

}


fn main() {
    let a = Whether1{
        high_temp: 10.0,
        low_temp: 20.0,
        chance_of_precipitation: 50.0
    };
    println!("{}", a.summary());
   Whether1::add();
    
   
}
