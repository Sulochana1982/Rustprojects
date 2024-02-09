fn main()
{
let mut a = [1, 2, 0, 4, 3, 0, 5, 0];
let n = a.len();
println!("{}",n);
for i in 0..n-1
{
    if a[i]==0
    {
        for j in i..n-1
        {
            a[j]=a[j+1];
        }
        a[n-1]=0;
    }
    println!("{:?}",a);
}
}
