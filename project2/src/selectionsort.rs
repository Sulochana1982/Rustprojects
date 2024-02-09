fn main()
{
    let mut a = [88, 33, 27, 14, 35, 19, 42, 44];
    let n = a.len();
    
    
    for i in 0..n-1
    {   
        let mut min = i;
        for j in i+1..n
        {
           
            if a[j]<a[min]
            {
              min = j;
            }
        }
        if min!=i{
            let temp = a[i];
            a[i]= a[min];
            a[min]= temp;
            
        }
    }
    println!("{:?} ", a);
}








fn main()
{
    let mut a = [14, 33, 27, 10, 35, 19, 42, 44, 7];
    let n = a.len();
    
    for i in 0..n-1
    {
        let mut minindex=i;
       for j in i+1..n
       {

        if a[j]<a[minindex]
        {
            minindex=j;
        }

       }
       if (i!=minindex)
       {
        let mut temp = a[i];
        a[i]=a[minindex];
        a[minindex]=temp;
       }

    }

    println!("{:?}", a);


}