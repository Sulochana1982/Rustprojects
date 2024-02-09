fn main()
{
   let n = 5;
   let big = if n <10 && n>-10
   {
      println!(" small number");
      10*n
   }
   else
   {
      println!("big number");
      n/2
   };
   println!("{} -> {}", n, big);
   
}