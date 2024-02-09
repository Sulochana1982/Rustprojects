fn main() {
    let a:i8 = 8;
    let b:i8 = 10;
     
   println!("The sum of the numbers is: {}", add(a,b) );

}

fn add(x:i8, y:i8) ->i8 {
    x+y
}