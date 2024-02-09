fn main()
{
   /*let mut a = 12;
    a=18;
    println!("{a}");
   let a=true;
    println!("{a}");

    println!("1+2 = {}", 1 + 2);*/

   let mut n = 10;
   {
    println!("{n}");
    let m=20;
    println!("{m}");
   }

   //println!("{m}");
   println!("{n}");
   let n=100;
   println!("{n}");


}

