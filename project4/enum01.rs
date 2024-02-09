#[derive(Debug)]
enum Number{
    Zero,
    One,
    Two,
}
#[derive(Debug)]
enum Number1{
    Zero=0,
    One,
    Two,
}
#[derive(Debug)]
enum Number2
{
    Zero = 0,
    One = 1,
    Two =2,
}

fn main()
{
    assert_eq!(Number::Zero as u8, Number1::Zero as u8);
    println!("success");

}