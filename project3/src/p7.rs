

#[derive(Debug)]

struct Rectangle{
    width : u32,
    height : u32,
 }
 
 fn main()
 {
    let mut r1 : Rectangle = Rectangle{
       width: 3,
       height: 5,
    };
 
    println!("{}", area(&mut r1));

    println!("{:?}",r1 );
    
 }
 
 fn area(r:&mut Rectangle) -> u32{
   r.width = 10;
   r.height = 10;
    r.width * r.height
 }
