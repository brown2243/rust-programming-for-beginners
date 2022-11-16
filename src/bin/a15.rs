// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Vip(i32),
    Standard(i32, String),
    Backstage(i32, String),
}
fn main() {
    let vip = Ticket::Vip(500);
    let standard = Ticket::Standard(300, "king".to_string());
    let backstage = Ticket::Backstage(100, "queen".to_string());

    let people = vec![vip, standard, backstage];
    for person in people {
        match person {
            Ticket::Vip(300) => println!("{}", "lie"),
            Ticket::Vip(amount) => println!("{}", amount),
            Ticket::Standard(amount, name) => println!("{} , {}", amount, name),
            other => println!("{:#?}", other),
        }
    }
}
