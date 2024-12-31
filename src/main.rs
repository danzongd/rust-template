#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    sex: String
}

impl Person {
    fn new(name: String, age: u8, sex: String) -> Person {
        Person {
            name,
            age,
            sex
        }
    }
}

fn main() {
    let a = Person::new("John".to_string(), 25, "male".to_string());
    println!("{:#?}", a);
}