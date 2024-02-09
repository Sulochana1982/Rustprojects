


// fn hell()
// {

// let nums = vec![1, 2, 3];

// for i in 0..nums.len() {
//     println!("{}", nums[i]);
// }



// let nums = vec![1, 2, 3];

// for num in &nums {
//     println!("{}", num);
// }



// let nums = vec![1, 2, 3];

// for num in &nums {
//     println!("{}", *num);
// }
// }

// fn main() {
//     #[derive(Debug)]

//     struct User {
//          user:String,
//          email:String,
//          password:String,
//     }
    

//     let mut user_1 = User {
//         user: String::from("Hello, I am user 1"),
//         email: String::from("emailid@gmail.com"),
//         password: String::from("labadaba"),
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         user: user_1.user.clone(),
//         password: user_1.password.clone(),
//     };

//     println!("{:?}", user_1);
//     println!("{:?}", user2);
// }


fn main()
{
    for number in (1..4).rev(){
        println!("{}", number);
    }
}
