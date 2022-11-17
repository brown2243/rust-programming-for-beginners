// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}
fn check_age(customer: &Customer) -> Result<String, String> {
    if customer.age > 20 {
        Ok("SUCCESS".to_owned())
    } else {
        Err("Faile".to_owned())
    }
}
fn main() {
    let brown = Customer { age: 30 };

    let res = check_age(&brown);
    println!("{:#?}", res);
    match res {
        Ok(ans) => println!("{}", ans),
        Err(error) => println!("{}", error),
    }
}
