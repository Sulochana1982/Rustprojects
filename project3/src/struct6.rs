#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {

fn color(&self) -> &str

{
    match self
    {
    //TrafficLightColor::Yellow => "yellow",
    Self::Yellow => "yellow",
    _ => "red",
        
    }
}
    
}

fn main() {
    let c : TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
