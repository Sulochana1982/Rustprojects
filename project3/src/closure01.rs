fn main()
{
    let  color = String :: from("green");

    //let print = move || println!("color: {}", color);
    let print =  || println!("color: {}", color);
    print();
    print();
    print();


   // let _reborrow = &color;

   //println!("{}", color);
}