fn main()
{
    let mut a = [10, 21, 22, 100, 101, 200, 300];
    let n = a.len();
    let mut count =0;
    for i in 0..n-2
    {
       for j in i+1..n-1
       {
        for k in j+1..n
        {
            if a[i]+a[j]>a[k] &&  a[j]+a[k]>a[i] && a[k]+a[i]>a[j]
            {
            println!("{}   {}   {}", a[i],a[j],a[k]);
            count= count +1;
            }
        }
       }
    }
    println!("Count: {}",count);
    

}