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
// Struct with aList in it
#[derive(Debug)]
struct List(Vec<i32>);


impl fmt::Display for List {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // Extract the value using Tueple Indexing and create a refference to the vector
        let vec = &self.0;
        write!(f, "[")?;
        // Iterating over the Vector while enumerating the iteration

        // No clue what enumerating the Iteration means. Doesnt Matter for now

        for (count,v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{1}",count,v)?;
        }

        write!(f, "]")
    }
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
    let complex = ComplexNumbers { 
        x:10,
        y:20
    };

    let ve  =  List(vec![1,2,3,4,5,6,7]);
    println!("{}",ve)
}
