use std::thread;
use std::time::Duration;
fn main()
{
    let user_value= 10;
    let random_number = 7;
    generate_workout(user_value, random_number);
    
}


fn generate_workout(intensity: i32, random_number: i32)
{
    let  expensive_result = |num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    if intensity<25{
        println!("Today do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    }
    else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated");
        }
        else{
            println!("Today, run for {} minutes", expensive_result(intensity))
        }
    }
}