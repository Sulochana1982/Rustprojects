

fn main()
{
    let o = Some(7);

    match o{
        Some(i) => {println!("The value of i is {:?}", i);
                     println!("success");},

                     
           _ => {}
    };


    if let Some(i) = o
    {
        println!("The value of i is {:?}", i);
        println!("Success");
    }

}           