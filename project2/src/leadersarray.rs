fn main()
{
    let a = [16, 17, 4, 3, 5, 2];
    let n = a.len();
    for i in 0..n
    {
        let mut leader = false;
        for j in i+1..n
        {
          
            if a[j]>=a[i]
            {
                leader= false;
                break;

            }
            else
            {
                leader = true;
            }
            if j==n-1
            {
                println!("{}   ",a[i]);
            }
            
    }

        if leader == true{
           println!("{}   ",a[i]);
        }
      

    }
}