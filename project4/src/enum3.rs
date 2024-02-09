#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main()
{

    let coin  = Coin::Penny;
    match coin{
        Coin :: Penny => 1,
        Coin :: Nickel => 5,
        Coin :: Dime => 10,
        Coin :: Quarter => 25,
    };

    println!("{:?}", coin);


}