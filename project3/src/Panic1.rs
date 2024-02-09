fn drink(beverage: &str){
    if beverage == "lemon"{

        println!("success");
        panic!();
    }
    println!("Dont print me");
}

fn main()
{
    drink("lemon");
    println!("Exercise failed if printing this line");

}