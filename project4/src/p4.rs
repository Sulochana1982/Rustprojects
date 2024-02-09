
#[derive(Debug)]

struct User{
   active: bool,
   username:String,
   email:String,
   sign_in_count:u64,
}
fn main()
{

   let email : String = String :: from("sulochana@gmail.com");
   let active : bool = false;
   let username : String = String :: from ("sulochana");
   let sign_in_count : u64 = 7; 


 
let mut u1:User = call(email, active, username, sign_in_count);


 u1.active = true;

println!("{}", u1.active);
println!("{}", u1.username);
println!("{}", u1.email);
println!("{}", u1.sign_in_count); 

  
   
}

fn call(email:String, active:bool, username:String, sign_in_count:u64)->&User

{

   let u1 : User = User{
      active,
      username,
      email,
      sign_in_count,
    };
    println!("{:?}", u1);
    u1


}
