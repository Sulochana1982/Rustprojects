#[derive(Debug)]
struct Rectangle{
   width: u32,
   height: u32,
}

impl Rectangle{
   fn area(&self) -> u32
   {
      self.width * self.height
   }
}

fn main()
{
   let r1: Rectangle = Rectangle{
      width : 5,
      height : 10,
   };

   println!("{}", r1.area());

}