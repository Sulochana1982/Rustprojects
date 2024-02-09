struct Rectangle {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    }
    
impl Rectangle{

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {

    let r1: Rectangle = Rectangle{
        width: 30,
        height: 20,
    };
    
    println!("{}", r1.area());
    
    let r2: Rectangle = Rectangle{
        width: 3,
        height: 2,
    };
    
    let b : bool;
    
    b = r1.can_hold(&r2);
    
    println!("{}", b);
    
    println!("Success!");
}
