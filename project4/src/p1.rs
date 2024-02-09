
#[derive(Debug)]

struct User{
   active: bool,
   username:String,
   email:String,
   sign_in_count:u64,
}
fn main()
{
    let n = 5;
    let m = 0.5;

 let u1: User = User{
   active: true,
   username: String::from("sulochana"),
   email: String::from("sulochana@gmail.com"),
   sign_in_count : 2,
 };
let u2: User = User{
   ..u1
}


println!("{}", u1.active);
println!("{}", u1.username);
println!("{}", u1.email);
println!("{}", u1.sign_in_count); 

println!("{:?}", u1);
println!("{:?}", u2);
  
   
}
