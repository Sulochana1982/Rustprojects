fn main()
{
    let mut sum = 0;
    for i in -3..2
    {
        // -3-2-1+1
        sum = sum + i;

    }

    assert!(sum == -5);

    for c in 'a'..='z'
    {
        println!("{}", c);
    }
}