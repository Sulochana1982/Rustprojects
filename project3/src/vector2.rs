fn main()
{
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2= Vec::new();
    v2.extend(&v1);
    //v2.extend(v1.clone());

    assert_eq!(v1, v2);

    println!("success");

    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);


    let mut v2:Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    assert_eq!(v1, v2);

   println!("success");

    
}
