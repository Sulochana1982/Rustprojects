


use std::io;

fn main() {

    let mut big = 0; 
    let mut count = 0;
    let mut check = 0;

    let mut total = String::new(); 

    println!("Please enter the total amount of numbers");
    io::stdin()
    .read_line(&mut total)
    .expect("Failed to read line");

    let total: usize = total
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let mut c = String:: new();
    println!("Please enter the groups the numbers are to be compared");
    io::stdin()
    .read_line(&mut c)
    .expect("Failed to read line");

    let c: usize = c
    .trim()
    .parse()
    .expect("Index entered was not a number");

    loop {

    let mut number = String::new();    
    print!("");
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let mut number: usize = number
    .trim()
    .parse()
    .expect("Index entered was not a number");
    
    count = count + 1;

    if number > big {
          big = number;
    }
     
    if count % c == 0 {
        println!("The biggest number is: {}", big);
        big = 0;
    }

  

    check = check + 1;

    if check == total {
        break;
    }

}
}