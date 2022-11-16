#[derive(Debug)]
enum Discount {
    Percent(i32),
    Flat(i32),
}
struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("{}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("{}", "Two"),
        Discount::Flat(amount) => println!("{}", amount),
        Discount::Percent(amount) => println!("{}", amount),
        _ => (),
    }
    println!("{:?}", flat);

    let concert = Ticket {
        event: "concert".to_string(),
        price: 50,
    };

    match concert {
        Ticket { price: 50, event } => println!("price is 50 {}", event),
        Ticket { price, .. } => println!("price {}", price),
        // _ => (),
    }
}
