fn main()
{
    let mut s = String::from("hello world");
    let world = first_word(&s);
    println!("{}", world);

    s.clear();
}

fn first_word(s: &String)-> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}



// fn main()
// {
//     let mut s = String::from("helloworld");
//     let world :&str= first_word(&s);
//     println!("{}", world);

//     s.clear();
// }

// fn first_word(s: &String)-> &str{
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return &s[0..i];
//         }
//     }
//     &s[..]

//}