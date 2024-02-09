use std::mem::size_of_val;
fn main()
{
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    println!("success");

    let x = 10;
    let y = x;
    println!("{}", y);
    

}