fn main()
{
    let  v:Vec<i32> = vec![1, 2, 3];
    let slice1:&[i32] = &v[..];
    let slice2:&[i32] = &v[0..v.len()];
    assert_eq!(slice1, slice2);
    println!("success");
    println!("{:?}", slice1);
    println!("{:?}",slice2);

    

}