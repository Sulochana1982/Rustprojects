fn main()
{
    let a = [ 10, 4, 3, 50, 23, 90];
    let mut first = std::i32::MIN;
    let mut second = std::i32::MIN;
    let mut third = std::i32::MIN;
     let n = a.len();
    for i in 0..n
    {
       if a[i]>first
       {
        third= second;
        second= first;
        first = a[i];
       }
       else if a[i]>second && a[i]!=first
       {
        third=second;
        second=a[i];


       }
       else if a[i]>third && a[i]!=second{
        third = a[i];
       }
    }

    println!("{}     {}     {}", first, second, third);
}