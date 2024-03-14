#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

trait Speak {
    fn say_hello(&self) -> String;
}

impl Speak for Person {
    fn say_hello(&self) -> String {
        format!("Hello, My name is {} and I am {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Jinnal Parakh"),
        age: 29,
    };
    println!("{:?}", person.say_hello());
}
