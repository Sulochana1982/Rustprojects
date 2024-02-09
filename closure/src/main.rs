// fn main() {
//     let mut num = 5;
//     let plus_num = |x: i32| x + num;
    
//     let y = &mut num;
//     println!("{}", y);
// }


use chrono::{Local};

fn main() {
    let now = Local::now();
    println!("Current date and time: {}", now);
}