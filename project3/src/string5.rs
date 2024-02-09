fn main()
{
    let mut s: String = String::from("hai");
    println!("{}", s);
    println!("{}", s);
    
    s = "fellow";
    s = s.to_string();
   // s= s.to_owned();
   //s= String::from(s);
    println!("{}", s);
    println!("{}", &s);
}