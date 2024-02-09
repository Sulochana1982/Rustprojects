
fn main()
{

let s = String::from("hello world");

let a = &s[0..5];
let b = &s[6..11];

println!("{}", a);
println!("{}", b);



let s = String::from("hello");

let slice1 = &s[0..2];
let slice2 = &s[..2];
println!("{}", slice1);
println!("{}", slice2);


let s = String::from("hello");

let len = s.len();

let slice1 = &s[3..len];
let slice2 = &s[3..];
let slice3 = &s[..];

println!("{}", slice1);
println!("{}", slice2);
println!("{}", slice3);




}


fn main()
{
    let  s = String::from("hello, world");

    let slice1: &str = s.as_str();
    let slice2 : &str = &s[0..5];//offset not index 
    //let slice2 : &str = &s[..5];

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