pub mod client;
pub mod network; 
// mod client{
//     fn connect()
//     {

//     }
// }
// mod network{
//     fn connect()
//     {

//     }
//     mod server{
//         fn connect()
//         {
            
//         }
//     }
// }

#[cfg(test)]
mod tests{

    use super::client;
    #[test]
    fn it_works(){
        client::connect();
    }
}
