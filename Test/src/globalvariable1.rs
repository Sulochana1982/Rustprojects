
use std::io;
static mut A: i32 = 0;
static mut B: i32 =0;

fn main() {

    let mut num1 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    let num1: i32 = num1.trim().parse().expect("Please type a number!");

    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num2: i32 = num2.trim().parse().expect("Please type a number!");



    unsafe {
    A = num1;
    B= num2;
   
    }

    let c = add();
    
    unsafe{
    println!("{}, {}, {}", A, B, c);
    }
    
}

fn add() -> i32
{

    unsafe {
        A+B
    }
  
}
