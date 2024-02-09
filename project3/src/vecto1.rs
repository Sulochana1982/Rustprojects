fn main()
{
    let mut v: Vec<i32> = Vec :: new();
    v.push(5);
    v.push(10);
    v.push(15);
    v.push(20);

    println!("{:?}", v);


    let v: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", v);
 
    let third: i32 = v[2];
    println!("Third element is {}:", third);

    match v.get(100){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no such element"),
    }

    let mut v: Vec<i32> = vec![10, 20, 30, 40];
    println!("{:?}", v);

     for i in &v{
        
        println!("{}", i);
     }

     println!("");

     println!("");
     println!("");
     println!("");


     
     for i in &mut v{
        *i = *i+10;
        
        println!("{}", i);
     }
     println!("{:?}", v);


}