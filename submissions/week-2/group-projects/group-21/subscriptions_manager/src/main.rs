use ::chrono::{DateTime, NaiveDate, Utc};
use ::std::io;

struct Subscription {
    service_name: String,
    cost: f64,
    renewal_date: DateTime<Utc>,
}

fn add_subscription(subscriptions: &mut Vec<Subscription>) {
    println!("\n--- Add New Subscription ---");

    // Get service name
    println!("Service Name: ");
    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("Failed to read input");
    let service_name = service_name.trim().to_string();

    // Get Service cost
    println!("\nMonthly Cost: $");
    let mut cost_input = String::new();
    io::stdin()
        .read_line(&mut cost_input)
        .expect("Failed to read input");
    let cost: f64 = match cost_input.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid cost! Please enter a number.");
            return;
        }
    };

    // Get renewal date
    println!("\nRenewal date (YYYY-MM-DD): ");
    let mut date_input = String::new();
    io::stdin()
        .read_line(&mut date_input)
        .expect("Failed to read input");

    // Parse date with UTC
    let renewal_date = match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
        Ok(date) => date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
        Err(_) => {
            println!("Invalid date format! Please use YYYY-MM-DD.");
            return;
        }
    };

    let subscription = Subscription {
        service_name,
        cost,
        renewal_date,
    };

    subscriptions.push(subscription);
    println!("Subscription added successfully!");
}

fn view_subscriptions(subscriptions: &Vec<Subscription>) {
    println!("\n--- All Subscriptions ---");

    if subscriptions.is_empty() {
        println!("No subscriptions found.");
        return;
    }

    for (i, sub) in subscriptions.iter().enumerate() {
        println!(
            "{}. {} - ${:.2}/month - Renews: {}",
            i + 1,
            sub.service_name,
            sub.cost,
            sub.renewal_date.format("%Y-%m-%d")
        );
    }
}

fn main() {
    let mut subscriptions: Vec<Subscription> = Vec::new();

    loop {
        println!("\nHello, Customer. What do you want to do today?\n");

        print!("1. Add Subscription\n2. View Subscriptions\n3. Exit\n");
        let mut buffer = String::new();

        println!("Choose an action (1/2/3): ");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        match buffer.trim() {
            "1" => add_subscription(&mut subscriptions),
            "2" => view_subscriptions(&subscriptions),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}
