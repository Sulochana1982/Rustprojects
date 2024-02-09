

use std::cmp::PartialOrd;
fn main()
{
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("{}", result);
    assert_eq!(result, 100);

    let char_list = vec!['a', 'b','c','d'];
    let result = largest(&char_list);
   
    assert_eq!(result, 'd');
    println!("{}", result);
}


fn largest<T:PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item> largest{
            largest = item;
        }
    }
    largest
}