use std::fmt::{self, write};



#[derive(Debug)]
struct Perrson{
    name: String,
    age: u8
}

impl fmt::Display for Perrson {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "My name is {1} and my age is {0}",self.age,self.name)
    }
}
fn main() {
    // Single Line Comment
    // println!("Hello, world!");
    // println!("Hellow I'm a Rustacean");
    // println!("{} days",31);
    // println!("My name is {1}. I am a {2}. {} am trying to become good at programing.","I","Nishchay","App Dev");
    // println!("BASE 10 : {}", 69420);
    // println!("BASE 2  : {:b}", 69420);
    // println!("BASE 8  : {:o}", 69420);
    // println!("BASE 16 : {:x}", 69420);
    // #[derive(Debug)]
    // struct Structure(i32);
    // println!("Structure : {:?}",Structure(56));
    // println!("Structure : {:?}",Structure(3));
    let person = Perrson{
        name:String::from("Nishchay"),
        age:24
    };
    println!("{}",person);

}
