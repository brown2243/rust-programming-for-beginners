// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let point1 = (0, 5);
    let point2 = (0, 4);
    let point3 = (0, 6);
    check(point1);
    check(point2);
    check(point3);
}

fn check(point: (i32, i32)) -> (i32, i32) {
    let (x, y) = point;

    if y > 5 {
        println!("great");
    } else if y < 5 {
        println!("less");
    } else {
        println!("equal");
    }

    (x, y)
}
