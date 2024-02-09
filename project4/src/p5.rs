


#[derive(Debug)]

struct R{
  s1: String,
  s2: String,

}

fn main()
{
   let r1 : R = R{
      s1 : String :: from("hello"),
      s2 : String :: from("hai"),
   };

  area(&r1);
  println!("{:?}", r1);
   
}

fn area(r: &R) {
println!("{:?}" , r);
   
}