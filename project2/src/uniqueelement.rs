fn main()
{
    let a = [12, 10, 9, 45, 2, 10, 10, 45, 12];
    let n= a.len();
    for i in 0..n
    {
        let mut b = false;
        for j in 0..n
        {
            if i==j
            {
            continue;
            }
            if a[i]==a[j]
            {
                b = true;
            }

        }
        if b == false{
            print!("{}   ",a[i]);
        }
    }
}