struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &Self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
   pub fn change_state(self: &mut Self) {
        self.color = "green".to_string();
        println!("{}", self.color);
    }
}
fn main() {

    let mut t1: TrafficLight = TrafficLight{
        color: "red".to_string(),
        
    };
    
    t1.show_state();
    t1.change_state();
    println!("Success!");
}
