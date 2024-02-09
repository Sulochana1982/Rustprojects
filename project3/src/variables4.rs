fn main()
{
    println!("{} {}", define_x(), "world");
}
fn define_x() -> String
{
    let x: String = String:: from ("hello");
    x
}