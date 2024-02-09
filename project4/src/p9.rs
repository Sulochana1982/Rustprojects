struct User{
    active: bool,
    username:&str,
    email:&str,
    sign_in_count:u64,
 }

 fn main()
 {
    let u1 = User{
        active: true,
        username: "hello",
        email: "hello@gmail.com",
        sign_in_count:7
    }
    println!("{:?}", u1);

 }