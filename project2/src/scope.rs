fn main() {
    
    let  x = 9;
      let x = x+1;
      println!("The value of x is {x}");
     {
       let x = x * 2;
        println!("The value of x is : {x}");
     }
    
      println!("The value of x is : {x}");
   
}



fn main()
{
    let tub = (500, 6.4, 1);
    let (x, y ,z) = tub;
    println!("The value of x : {x}");
}

use std::io;

fn main()
{
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
 
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Error");

    let index:usize = index.trim().parse().expect("error");
    let element = a[index];

    println!("element :{element}");


}






