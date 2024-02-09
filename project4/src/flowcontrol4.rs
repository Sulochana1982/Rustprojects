fn main()
{
   let names:[String;2] = [String::from("living"), String::from("hai")];
   for name in &names{
      println!("{}", *name);
   }
   println!("{:?}", names);






   let numbers: [u32;3]= [1, 2, 3];
   for num in numbers{
      println!("{}", num);
   }
   println!("{:?}", numbers);


}