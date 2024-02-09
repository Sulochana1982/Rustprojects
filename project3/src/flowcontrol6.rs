fn main()
{
   let mut n =1;

   while n < 10
   {
      if n%15 ==0{
         println!("fizz");
      }else if n%3==0
      {
         println!("buzz");
      }
      else{
         println!("{}", n);
      }

      n=n+1;
   }
}