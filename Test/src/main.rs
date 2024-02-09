



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


// Error
//println!("{}", r1);  


let r4 = &mut s;

println!("{}" , r4);


let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);

println!("{:?}", v);

// error
//println!("{}", first);

let len = String::from("    ").len();
println!("{}", len);


}