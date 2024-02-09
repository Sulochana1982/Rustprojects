



#[derive(Debug)]
enum IpAddrKind
{
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main()
{
    let home = IpAddr{
        kind : IpAddrKind::V4,
        address : String :: from("127.0.0.1"),

    };

    let loophole = IpAddr{
        kind : IpAddrKind::V6,
        address : String :: from("127.0.0.2"),

    };

    println!("{:?}", home.kind);
    println!("{:?}", home.address);

    println!("{:?}", loophole.kind);
    println!("{:?}", loophole.address);

    println!("{:?}", home);
    println!("{:?}", loophole);
}