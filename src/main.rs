use std::fmt::{self};



#[derive(Debug)]
struct Person{
    name: String,
    age: u8
}
#[derive(Debug)]
struct ComplexNumbers {
    x:i32,
    y:i32,
}

impl fmt::Display for ComplexNumbers {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} + {1}i",self.x,self.y)
    }
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
    let complex = ComplexNumbers { 
        x:10,
        y:20
    };
    println!("{}",complex);
}
