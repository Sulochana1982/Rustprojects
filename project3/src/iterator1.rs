fn main()
{
    let v1: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v1);

    let v1_iter = v1.iter();
    println!("{:?}", v1_iter);

    for val in v1.iter(){
        println!("{}", val);
    }
}


// fn main()
// {
//     let a = [10, 20, 30, 40, 50, 60];

//     for e in a.iter(){
//         println!("{}", e);
//     }
// }



// fn main()
// {
//     let v1: Vec<i32> = vec![1, 2, 3];
//     println!("{:?}", v1);

//     let v1_iter = v1.iter();
//     println!("{:?}", v1_iter);

//     for val in v1_iter{
//         println!("{}", val);
//     }
// }
