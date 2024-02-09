fn main()
{
   let a :[u32;4]= [4, 3, 2, 1];
   for (i, v) in a.iter().enumerate()
// enumerate take collection and return tuple index and value
   {
       println!("The {}th element is  {}", i+1, v);
   }
}