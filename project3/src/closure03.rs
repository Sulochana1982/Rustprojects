fn main(){
    let movable : Box<i32> = Box::new(3);
    let consume = ||{
        println!("movable {:?}", movable);
        take(movable);
    };
    consume();
    //consume();
   // println!("{}", movable);

}

fn take<T>(_v : T){}




// fn main(){
//     let movable : Box<i32> = Box::new(3);
//     let consume = ||{

//   closure will capture movable as immutable reference

//         println!("movable {:?}", movable);
//         take(&movable);
//     };
//     consume();
//     consume();

// }

// fn take<T>(_v : &T){}




