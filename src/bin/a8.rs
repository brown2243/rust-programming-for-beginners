// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparking,
    Sweet,
    Fruity,
}
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
struct Drinks {
    coke: String,
    soju: &'static str,
}
fn main() {
    let drinks = Drinks {
        coke: "black sweet".to_string(),
        soju: "damm shit",
    };
    println!("{},{}", drinks.soju, drinks.coke);

    let cooord = (2, 3);
    println!("{} , {}", cooord.0, cooord.1);
    let (x, y) = cooord;
    println!("{} , {}", x, y);
}
