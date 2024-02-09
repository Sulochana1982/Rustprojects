fn main()
{
    let  s = String::from("hello, world");

    let slice1: &str = s.as_str();
    let _slice2 : &str = &s[0..5];//offset not index 
    let slice2 : &str = &s[..5];

    println!("{}", slice1);
    println!("{}", slice2);

   // let mut slice3 : &str = &s;
    //slice3= slice3.to_owned() + "!";
    let mut slice3: String = String::from("hello, world");
    //let slice3: &mut String = &mut s;
    let mut slice4: String = s;
    slice3.push_str("!");
    slice4.push_str("!");
    assert_eq!(slice3, "hello, world!");
    assert_eq!(slice4, "hello, world!");
    println!("success");

}