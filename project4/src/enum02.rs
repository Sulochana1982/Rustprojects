
#[derive(Debug)]

enum Message
{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),

}
fn main()
{
     let msg1: Message = Message::Move{x:5, y:2};
     let msg2: Message = Message::Write(String::from("hello world"));
     println!("{:?}", msg1);
     println!("{:?}", msg2);
     
}
