fn main()
{
    //let mut v1 = Vec::new();
     //v1.push(6);
     //v1.push(7);
     //v1.push(9);
     //println!("{:?}",v1);

     //let v = vec![1, 2, 3, 4, 5];
     //let third: &i32 = &v[2];
     //println!("The third element is: {third}");
     //let third: Option<&i32> = v.get(80);
     //println!("The third element is: {:?}",third);

     let mut v = vec![100, 20, 90];
     for i in &mut v{
           *i += 50;
           println!("{i}");
     }
     println!("{:?}", v);


}