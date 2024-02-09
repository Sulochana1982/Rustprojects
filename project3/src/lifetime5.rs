struct A<'a>{
    part: & 'a str,
}

impl <'a> A<'a> {
    fn level(&self)-> i32{
        6
    }
}
fn main(){

}



struct A{
    part: & 'static str,
}

impl  A{
    fn level(&self)-> i32{
       
        6
        
    }
}
fn main(){

}