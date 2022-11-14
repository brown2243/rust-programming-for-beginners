// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Apple {
    id: i32,
    quantity: i32,
}
fn display_apple(apple: &Apple) {
    println!("{}, {}", apple.id, apple.quantity);
}
fn main() {
    let apple = Apple {
        id: 0,
        quantity: 9999,
    };
    display_apple(&apple);
    display_apple(&apple);
}
