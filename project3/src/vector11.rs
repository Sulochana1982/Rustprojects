fn main()
{
    let arr:[u8; 3] = [1, 2, 3];

    let v : Vec<u8> = Vec::from(arr);

    println!("{:?}", v);

   call1_02(v);

    let v = vec![1, 2, 3];
    call1_02(v);

    let v = vec!(1, 2, 3);
    call1_02(v.clone());

    let mut v1 = Vec::new();

    for i in &v{
        v1.push(*i);
    }

    call1_02(v1.clone());

    assert_eq!(v, v1);
    println!("success");
}

fn call1_02(v: Vec<u8>)
{

}