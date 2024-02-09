struct Owner(i32);

impl Owner{
    fn add_one(&mut self)
    {
        self.0= self.0 +1;
        println!("Print :   {}  ", self.0);
    }

    fn print(&self)
    {
        println!("Print :   {}  ", self.0);

    }
}

fn main()
{
    let mut owner: Owner = Owner(20);
    owner.add_one();
    owner.print();
}