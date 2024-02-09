pub mod network{
    pub fn connect()
    {
        println!("I am in network module");
    }
    pub mod client{
       pub fn connect()
        {
            println!("I am in client module");
        }
    }
}

