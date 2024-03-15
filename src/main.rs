#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

trait Speak {
    fn introduce(&self) -> String {
        "Hello, My name is rahul and I am 21 years old".to_string()
    }

    fn reply(&self,_other:Person) -> String{
        "Nice, to meet you".to_string()
    }
}

impl Speak for Person {
    fn introduce(&self) -> String {
        format!("Hello, My name is {} and I am {} years old", self.name, self.age)
    }

    fn reply(&self,other:Person) -> String {
        format!("Hello, Nice to meet you {}.My Name is {} and I am {} years old", other.name,self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Jinnal Parakh"),
        age: 29,
    };
    println!("{:?}", person.introduce());

    let person1 = Person {
        name: String::from("Rohit Bhandari"),
        age: 29,
    };

    println!("{:?}", person1.reply(person));

    let mut a = 1;
    let mut b = 2;
    let mut c = a + b;
    println!("{:?}", c);
}
