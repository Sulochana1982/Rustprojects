fn main(){
    let x =7;
    let y = 9;
    print_one(&x);

    print_multi(&x, &y);
    
    let z : &i32 = pass_x(&x, &y);
    println!(" the value of z : {}  ", *z+1);

    print_one(&z);
    
    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn print_one(x: &i32)
{
    println!("Print_one : x is  :{}  ", x);
}

fn print_multi(x: &i32, y: &i32)
{
    println!("Print_multi : {}   {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, y: & 'b i32) -> & 'a i32
{
    &x
}
fn add_one(x: &mut i32)
{
    *x += 1;
}