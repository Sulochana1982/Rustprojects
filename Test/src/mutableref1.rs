



fn main()
{
let mut s = String::from("hello");

 // no problem
let r2 = &s; // no problem

let r1 = &s;

println!("{}" , r1);

println!("{}" , r2);

// if one mutable ref occurs ... You cannot use previous references... and  between 
// mutable ref and that usage No reference(Mutable and immutable) are allowed...

let r3 = &mut s; // BIG PROBLEM

println!("{}" , r3);


let r4 = &mut s;

println!("{}" , r4);



}