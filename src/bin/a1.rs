// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let name = "brown Kim";
    println!("{:?}, {:?}", print_last_name(name), print_first_name(name));
}

fn print_first_name(name: &str) -> &str {
    name.split(" ").collect::<Vec<&str>>()[0]
}
fn print_last_name(name: &str) -> &str {
    name.split(" ").collect::<Vec<&str>>()[1]
}
