use std::collections::HashMap;
fn main()
{
    let mut scores: HashMap<&str, i32>  = HashMap::new();
    
    scores.insert("sunface", 98);
    scores.insert("Hari", 89);
    scores.insert("Nani", 78);

    let score = scores.get("sunface");
    assert_eq!(score, Some(&98));
    println!("{:?}", score);


    if scores.contains_key("sunface")
    {
        let score:i32 = scores["sunface"];
        assert_eq!(score, 98);
        scores.remove("sunface");

    }

    assert_eq!(scores.len(), 2);
    
    for (name, score) in scores{
        println!("The score {} is {}", name, score);
    }


}