








fn main()
{
let mut x = 0..10;

loop {
    match x.next() {
        Some(x) => {
            println!("{}", x);
        },
        None => { break }
    }
}

for x in 0..10
{
println!("{}", x);
}
}