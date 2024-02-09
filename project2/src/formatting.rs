
    /*println!("{} days", 31);
    println!("{number:2>3}", number=1);
    println!("{number:5<6}", number=1);
    println!("{number:5<width$}",number=1, width=6);*/

    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    #[derive(Debug)]
    struct Person<'a>
    {
        name: &'a str,
        age:u8
    }
    fn main()
    {
        println!("{1:?} {0:?} is the {actor:?} name", "Jayaprakash","sulochana", actor = "actors's");
        println!("{:?}",Structure(3));
        println!("{:?}", Deep(Structure(6)));

        let name ="shiva";
        let age = 13;
        let s = Person {name, age};
        println!("{:#?}",s);

    }
