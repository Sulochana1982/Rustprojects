fn main()
{
    let a =[[1, 2, 3],[4, 5, 3], [2, 4, 5]];
    let b= [[1, 2, 3], [2 , 3, 2], [3, 4, 5]];
    let  mut c = [[0, 0, 0],[0, 0, 0], [0, 0, 0]];
    
    for i in 0..3
    {
        for j in 0..3
        {
            c[i][j]=0;
           for k in 0..3
           {
            c[i][j]=c[i][j] + a[i][k]*b[k][j];

           }
          
           
        }

    }
    println!("{:?}" ,a);
    println!("{:?}", b);
    println!("{:?}", c);
}