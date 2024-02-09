fn add(a: i32, b:i32) -> i32{
    a+b
}

#[cfg(test)]
mod test{
   use super::*;

   #[test]
   fn internal(){
    assert_eq!(4, add(2, 2))
   }
}