fn main() {
    let  a = [5, 3, 1, 4, 2, 6];
    let n = a.len();
    for i in 0..n
    {
        for j in i+1..n
        {
             if (a[i]+a[j])==10
        {
           println!("{} , {} are pair", a[i],a[j]);
        }
       
    }
}
   
}