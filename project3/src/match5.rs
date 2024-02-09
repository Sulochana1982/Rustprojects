enum MyEnum{
    Foo,
    Bar,
}

fn main()
{
    let mut count = 0;
    let v : Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v{
        if matches!(e, MyEnum::Foo)
        {
            count = count + 1;
        }

    }

    assert_eq!(count, 2);
    println!("success");

}