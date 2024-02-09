
// time book
// https://time-rs.github.io/book/


use time::OffsetDateTime;
fn main()
{
let now = OffsetDateTime::now_utc();
// let local = OffsetDateTime::now_local();

println!("{now}");
}

// cargo book

//https://doc.rust-lang.org/cargo/commands/cargo-publish.html