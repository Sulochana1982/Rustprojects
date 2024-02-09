fn main()
{
    let str1 : String = String::from("long string");
    {
        let str2: String = String::from("XYZ");
        let result : &str = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}   :", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str)-> &'a str{
    if x.len()>y.len()
    {
        x
    }
    else
    {
        y
    }
}