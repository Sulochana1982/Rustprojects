

#[derive(Debug)]

struct Rectangle{
    width : u32,
    height : u32,
 }
 
 fn main()
 {
    let r1 : Rectangle = Rectangle{
       width: 3,
       height: 5,
    };
 
    println!("{}", area(r1));

    println!("{:?}",r1 );
    
 }
 
 fn area(r:Rectangle) -> u32{
    r.width * r.height
 }