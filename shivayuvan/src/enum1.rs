// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {

//     let home:IpAddr = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback:IpAddr = IpAddr{
//         kind:IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("{:#?}   
//     {:#?}  ", home, loopback);
// }



// #[derive(Debug)] 
// enum UsState {
//     Alabama,
//     Alaska,
    
// }
// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn main() {
//     let c = Coin::Quarter(UsState::Alabama);


//    let d = value_in_cents(c);

//    println!("{d}");  

// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

