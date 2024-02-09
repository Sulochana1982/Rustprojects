fn main()
{
    let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);

}



fn main() {
    // let mut x = 5;
    // let y = &mut x;

    // *y += 1;

    // println!("{}", x);

    let y: &i32;
let x = 5;
y = &x;

println!("{}", y);
}