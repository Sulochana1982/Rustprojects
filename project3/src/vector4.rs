fn main()
{
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5
    {
        println!("{:?}", v.get(i));

    }
    for i in 0..5{
        match v.get(i)
        {
            Some(e) => v[i] = e+ 1,
            None => v.push(i+2),
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);
    println!("success");

}