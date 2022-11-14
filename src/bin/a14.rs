// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}
impl Person {
    fn new(data: (i32, String, String)) -> Self {
        let (age, name, favorite_color) = data;
        Self {
            age,
            name,
            favorite_color,
        }
    }
    fn print_info(&self) {
        println!("{},{},{}", self.age, self.name, self.favorite_color);
    }
}
fn print(data: &str) {
    println!("{}", data);
}
fn main() {
    let person1 = Person::new((30, String::from("brown"), "blue".to_owned()));
    let person2 = Person::new((5, String::from("derek"), "red".to_owned()));
    let person3 = Person::new((10, String::from("doggle"), "pink".to_owned()));
    let arr = vec![person1, person2, person3];

    for person in arr {
        if person.age <= 10 {
            person.print_info();
            print(&person.favorite_color);
            print(&person.name);
        }
    }
}
