fn main()
{
    let a = [-1, -1, 6, 1, 9, 3, 2, -1, 4, -1];
    let temp = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let n = a.len();
    
   
    for i in 0..n
    {
    for j in 0..n
    {
        if a[j]== i 
        {
            temp[i]= a[j];
        }
        else
        {
            temp[i]= -1;
        }
    }
    }
    println!("{:?}", temp);
}