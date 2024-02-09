fn main()
{
    let mut a = [10,5,30,3,50,8];
    let n = a.len();
    for i in 0..n
    {
        println!("{}", i);
        if i%2==1
        {
           if a[i]<a[i-1]
           {
           let temp = a[i];
         a[i]=a[i-1];
         a[i-1]=temp;
         print!("{}   {}", a[i],a[i-1]);
           }
        println!("");


            
        }
       
    }
    print!("{:?}   " , a);
}