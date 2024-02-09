fn main() {

 let mut s = String::from("hello");

let mut r1 = &mut s;
*r1 = String::from("haihai");

println!("{}", s);


let r2 =  &mut s;






println!("{}",  r2);


   





}