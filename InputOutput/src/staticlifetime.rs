fn main()
{
    let x: &str = add();
    println!("The value : {}", x);
}

fn add() -> & 'static str
{
    let s: & 'static str = "I have static life time";
    s

}