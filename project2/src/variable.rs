/*fn main()
{
    let n;
    {
        let x= 5;
        n = x * x;
        println!("{n}");
    }
    println!("{n}");
    //println!("{x}");
}*/

fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}
