enum Message
{
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main()
{
    let msgs = [Message::Quit, Message::Move{x:1, y:3}, Message::ChangeColor(255, 255, 0)];


for msg in msgs{
    show_message(msg);

}
}

fn show_message(msg: Message)
{
    match msg{
        Message::Move{x:a, y:b} => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
            println!("{} {}", a, b);
        },

        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
            println!("{}{}{}", r, g, b);
        },
        Message::Quit => println!("Quit"),

        Message::Write(a) => println!("No value"),


       // _ => println!("no data"),

    }
}