
#[allow(unused_variables)]
enum TrafficLight{
    Red,
    Yellow,
    Green,
}
use TrafficLight::{Red, Yellow};
#[allow(unused_variables)]
fn main()
{
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

}