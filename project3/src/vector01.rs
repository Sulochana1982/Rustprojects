 
//  fn main()
//  {

 
//  let mut v= vec![1, 2, 3, 4, 5];

// for i in &mut v {
//     *i+=50;
//     println!("{}", *i);
// }
// println!("{:?}", v);



//  }

fn main() {

    let arr: [u8; 3] = [1, 2, 3];

    let v: Vec<u8> = Vec::from(arr);
   // is_vec(v);


    let v1:Vec<u8> = vec![1, 2, 3];
   // is_vec(v1);

    //The above vectors are of the same type.

    assert_eq!(v, v1);
    println!("Success");

   // vec!(..) and vec![..] are same macros, so

    // let v:Vec<u8> = vec!(1, 2, 3);
    // is_vec(v.clone());

    // let mut v1 = Vec::new();

    // for i in &v{
    //     v1.push(*i);

    // }
   

    // is_vec(v1.clone());

    // assert_eq!(v, v1);

    // println!("Success");

}

//fn is_vec(_v: Vec<u8>) {}