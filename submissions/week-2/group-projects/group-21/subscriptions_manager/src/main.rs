use ::chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use ::std::io;

struct Subscription {
    service_name: String,
    cost: f64,
    renewal_date: DateTime<Utc>,
}

fn add_subscription(subscriptions: &mut HashMap<String, Subscription>) {
    println!("\n--- Add New Subscription ---");

    // Get service name
    println!("Service Name: ");
    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("Failed to read input");
    let service_name = service_name.trim().to_string();

        // Check if subscription already exists
        if subscriptions.contains_key(&service_name) {
            println!("Subscription for '{}' already exists!", service_name);
            return;
        }

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
        service_name: service_name.clone(),
        cost,
        renewal_date,
    };

    subscriptions.insert(service_name, subscription);
    println!("\nSubscription added successfully!");
}

fn view_subscriptions(subscriptions: &HashMap<String, Subscription>) {
    println!("\n--- All Subscriptions ---");

    if subscriptions.is_empty() {
        println!("No subscriptions found.");
        return;
    }

    let mut subs: Vec<&Subscription> = subscriptions.values().collect();
    subs.sort_by(|a, b| a.service_name.cmp(&b.service_name));
    
    for (i, sub) in subs.iter().enumerate() {
        let status = if sub.renewal_date < Utc::now() {
            " (EXPIRED)"
        } else {
            ""
        };
        
        println!("{}. {} - ${:.2}/month - Renews: {}{}", 
                 i + 1, 
                 sub.service_name, 
                 sub.cost, 
                 sub.renewal_date.format("%Y-%m-%d"),
                 status);
    }
}

fn remove_expired_subscriptions(subscriptions: &mut HashMap<String, Subscription>) {
    println!("\n--- Remove Expired Subscriptions ---");
    
    let current_time = Utc::now();
    let expired_services: Vec<String> = subscriptions
        .iter()
        .filter(|(_, sub)| sub.renewal_date < current_time)
        .map(|(key, _)| key.clone())
        .collect();
    
    if expired_services.is_empty() {
        println!("No expired subscriptions found.");
        return;
    }
    
    println!("Found {} expired subscription(s):", expired_services.len());
    for service in &expired_services {
        if let Some(sub) = subscriptions.get(service) {
            println!("- {} (expired: {})", service, sub.renewal_date.format("%Y-%m-%d"));
        }
    }
    
    println!("Remove all expired subscriptions? (y/n): ");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("Failed to read input");
    
    if confirm.trim().to_lowercase() == "y" {
        for service in expired_services {
            subscriptions.remove(&service);
        }
        println!("Expired subscriptions removed successfully!");
    } else {
        println!("Removal cancelled.");
    }
}

fn main() {
    let mut subscriptions: HashMap<String, Subscription> = HashMap::new();

    loop {
        println!("\nHello, Customer. What do you want to do today?\n");

        println!("1. Add Subscription\n2. View Subscriptions\n3. Remove Expired Subscriptions\n4. Exit");
        let mut buffer = String::new();

        println!("Choose an action (1/2/3/4): ");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        match buffer.trim() {
            "1" => add_subscription(&mut subscriptions),
            "2" => view_subscriptions(&subscriptions),
            "3" => remove_expired_subscriptions(&mut subscriptions),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}
