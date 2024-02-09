

struct Guess{
    value: u32,
}

impl Guess{
    pub fn new(value:u32) -> Guess{
        if value<10  {
           // panic!("Guess value must be less than 100: {}", value);
            panic!("Guess value must be less than or equal to 100");
        }
        else if value>100{
           // panic!("Guess value must be greater than 1 {}", value);
              panic!("Guess value must be less than or equal to 100");
        }
        else{
            Guess{
                value
            }
        }
        
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]

    fn greater_than_100(){
        Guess::new(2);
    }
}










































struct Guess{
    value: u32,
}

impl Guess{
    pub fn new(value:u32) -> Guess{
        if value<1  {
            panic!("Guess value must be less than 100: {}", value);
        }
        else if value>100{
            panic!("Guess value must be greater than 1 {}", value);

        }
        else{
            Guess{
                value
            }
        }
        
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]

    fn greater_than_100(){
        Guess::new(200);
    }
}





struct Guess{
    value: u32,
}

impl Guess{
    pub fn new(value:u32) -> Guess{
        if value<1  {
            panic!("Guess value must be less than 100: {}", value);
        }
        else if value>100{
            panic!("Guess value must be greater than 1 {}", value);

        }
        else{
            Guess{
                value
            }
        }
        
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]

    fn greater_than_100(){
        Guess::new(200);
    }
}