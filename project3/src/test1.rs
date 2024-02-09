#[derive(Debug)]
pub struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle{
    pub fn can_hold(&self, other:&Rectangle) -> bool{
        self.length>other.length && self.width>other.width
    }
}


#[cfg(test)]
mod tests{
    use super::*;

  #[test]
  fn larger_can_hold_smaller(){
    let larger = Rectangle{length:8, width:7};
    let smaller = Rectangle{length:3, width:4};
    assert!(larger.can_hold(&smaller));
  }  
}