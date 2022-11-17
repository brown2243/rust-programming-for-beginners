// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

/// doc test
struct Student {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let brown = Student {
        name: "brown".to_owned(),
        // locker: Some(9999),
        locker: None,
    };

    match brown.locker {
        Some(num) => println!("{}", num),
        None => println!("not have"),
    }
}
