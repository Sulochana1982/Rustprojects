use std::fs::File;
fn main()
{
    let f:u32 = File::open("hello.txt");
}



// let f:u32 = File::open("hello.txt");
// |           ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `Result<File, Error>`
// |           |
// |           expected due to this
// |
// = note: expected type `u32`
//            found enum `Result<File, std::io::Error>`



// use std::fs::File;
// fn main()
// {
//     let f  = File::open("Desktop/hello.txt");
//     let f = match f {
//         Ok(file)=> file,
//         Err(error) => {
//             panic!("Problem in opening file");

//         },
//     };
// }