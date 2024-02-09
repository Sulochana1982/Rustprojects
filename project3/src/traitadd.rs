
use std::ops::Add;


#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Point{
    x: i32,
    y: i32,
}


impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main()
{
    let a = Point{x: 3, y: 5};
    let b = Point{x: 30, y:50};
    let c = a+b;

    println!("{:?}", c);
}
