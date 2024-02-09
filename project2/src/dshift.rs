fn main()
{
    let mut a= [ 1, 2, 3, 4, 5, 6, 7];
    let n = a.len();
    let d = 2;
    let mut temp =0;
    for i in 0..d
    {
        temp=a[0];
        for j in 0..n-1{
        
           a[j]=a[j+1]

        }
        a[n-1]=temp;
    }
    println!("{:?}", a);

}