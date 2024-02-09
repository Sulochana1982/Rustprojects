fn main()
{
    let mut a = [14, 33, 34, 35, 38];
    let n = a.len();
    for i in 0..n
    {
        let mut b = false;
        for j in 0..n-1
        {
            if a[j]>a[j+1]
            {
                let temp=a[j];
                a[j]=a[j+1];
                a[j+1]=temp;
                b= true;
                println!("swap");
               
            }
            println!(" {:?}",a);
        }
        if b == false
        {
            
          break;
          
        }
        
       
        println!("");
    }
   
    
}