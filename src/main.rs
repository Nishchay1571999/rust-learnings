use std::fmt::{self, write};



#[derive(Debug)]
struct Person{
    name: String,
    age: u8
}

impl fmt::Display for Person {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "My name is {1} and my age is {0}",self.age,self.name)
    }
}
fn main() {
    let person = Person{
        name:String::from("Nishchay"),
        age:24
    };
    println!("{}",person);

}
