fn main()
{
    let mut count =0;
    // "move" takes copy for integers and float, for string it moves...
     
   // let mut inc =  || {

    let mut inc = move || {
        count += 1;
        println!("count: {}", count);

    };
    inc();
   inc();


   let  count_reborrow1 = &mut count; 
   *count_reborrow1+=1;
   println!("{}", count_reborrow1);


   let reborrow = &count;
   let m = &count;


   println!("{}", m);
   println!("{}", reborrow);




   let  count_reborrow = &mut count;
   *count_reborrow+=1;
   println!("{}", count_reborrow);

   // assert_eq!(count, 0);
    println!("{}", count);
    

}