fn main()
{
    let a = [1, 4, 20, 3, 10, 5, 31, 2, 33];
    let n=a.len();
    
    
    for i in 0..n
    {
        let mut sum = 0;
        for j in i..n
        {
            sum = sum + a[j];
            
            //print!("{}   ", sum);
            if sum>33
            {
                break;
            }
        
        if sum ==33
        {
            for k in i..j+1
            {
               println!("{}  ",a[k]);
            }
        }

       


    }

    println!("");
    }
}