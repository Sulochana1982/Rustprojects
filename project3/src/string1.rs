fn main()
{
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push_str("!");
    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");
    println!(" success");


}

fn move_ownership(s:String)
{
    println!("ownership of {}", s);
}