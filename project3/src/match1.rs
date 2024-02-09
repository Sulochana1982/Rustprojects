enum Direction
{
    East,
    West,
    North,
    South,
}

fn main()
{
    let dire = Direction::South;
    match dire{
        Direction::East => println!("East"),
        Direction::West|Direction::North => {println!("West or North");},
        Direction::South => println!("South"),
        _ => println!("None"), 
    }

}




#[allow(unused)]
enum Direction
{
    East,
    West,
    North,
    South,
    SouthEast,
}

fn main()
{
    let dire = Direction::South;
    match dire{
        Direction::East => println!("East"),
        Direction::West|Direction::North => {println!("West or North");},
        Direction::South => println!("South"),
        _ => println!("None"), 
    }

}