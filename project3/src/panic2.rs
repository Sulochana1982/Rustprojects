fn main()
{
    assert_eq!("abc".as_bytes(), [97, 98, 99]);
    println!("success");

    let v = vec![1, 2, 3];
    let ele = v[2];
    println!("success");

    let ele = v.get(2).unwrap();

    let v = add(2);


}

fn add(speed: u8) -> f64
{
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        _ => 0 as f64,
    }
}