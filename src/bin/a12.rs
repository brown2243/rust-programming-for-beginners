// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
#[derive(Debug)]
enum Color {
    Red,
    Blue,
}
struct Box {
    dimensions: i32,
    weight: i32,
    color: Color,
}
impl Box {
    fn create_box(data: (i32, i32, Color)) -> Self {
        let (dimensions, weight, color) = data;
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print_characteristics(&self) {
        println!("{},{},{:?}", self.dimensions, self.weight, self.color);
    }
}

fn main() {
    let blue_box = Box::create_box((4, 500, Color::Blue));
    let red_box = Box::create_box((4, 500, Color::Red));
    blue_box.print_characteristics();
    red_box.print_characteristics();
}
