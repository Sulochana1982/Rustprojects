fn main()
{
    let arr : [i32; 3] = [1, 2, 3];
    let v1 : Vec<i32>= Vec::from(arr);
    let v2 : Vec<i32> = arr.into();
    assert_eq!(v1, v2);
    println!("success");


    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();
    
    let s = "hello".to_string();
    let v2:Vec<u8> = s.into_bytes();

    assert_eq!(v1, v2);
    println!("success");


    let s: &str ="hello";
    let v3: Vec<u8>= Vec::from(s);
    assert_eq!(v2, v3);
    println!("success");
    //let v4: Vec<i32> = [0; 10]


}