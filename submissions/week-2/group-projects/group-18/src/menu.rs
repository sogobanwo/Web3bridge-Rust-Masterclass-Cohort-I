use crate::manager::FeedbackManager;
use crate::models::FeedbackError;
use crate::input::get_user_input;

pub fn add_feedback_menu(manager: &mut FeedbackManager) {
    println!("\n=== ADD FEEDBACK ===");
    
    let customer_name = match get_user_input("Enter customer name: ") {
        Ok(name) => name.trim().to_string(),
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };

    if customer_name.is_empty() {
        println!("Customer name cannot be empty.");
        return;
    }

    let comment = match get_user_input("Enter feedback comment: ") {
        Ok(comment) => comment.trim().to_string(),
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };

    if comment.is_empty() {
        println!("Comment cannot be empty.");
        return;
    }

    let rating_input = match get_user_input("Enter rating (1-5 stars): ") {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };

    let rating: u8 = match rating_input.trim().parse() {
        Ok(r) => r,
        Err(_) => {
            println!("Please enter a valid number for rating.");
            return;
        }
    };

    match manager.add_feedback(customer_name, comment, rating) {
        Ok(()) => println!("Feedback added successfully!"),
        Err(FeedbackError::InvalidRating) => println!("Rating must be between 1 and 5."),
        Err(_) => println!("Failed to add feedback."),
    }
}

pub fn remove_feedback_menu(manager: &mut FeedbackManager) {
    println!("\n=== REMOVE FEEDBACK ===");
    manager.list_feedback_ids();
    
    if manager.is_empty() {
        return;
    }

    let id_input = match get_user_input("Enter feedback ID to remove: ") {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };

    let id: u32 = match id_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Please enter a valid feedback ID.");
            return;
        }
    };

    match manager.remove_feedback(id) {
        Ok(()) => println!("Feedback removed successfully!"),
        Err(FeedbackError::FeedbackNotFound) => println!("Feedback with ID {} not found.", id),
        Err(_) => println!("Failed to remove feedback."),
    }
}

pub fn edit_feedback_menu(manager: &mut FeedbackManager) {
    println!("\n=== EDIT FEEDBACK ===");
    manager.list_feedback_ids();
    
    if manager.is_empty() {
        return;
    }

    let id_input = match get_user_input("Enter feedback ID to edit: ") {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };

    let id: u32 = match id_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Please enter a valid feedback ID.");
            return;
        }
    };

    match manager.edit_feedback(id) {
        Ok(()) => {},
        Err(FeedbackError::FeedbackNotFound) => println!("Feedback with ID {} not found.", id),
        Err(FeedbackError::InvalidRating) => println!("Rating must be between 1 and 5."),
        Err(FeedbackError::InvalidInput) => println!("Invalid input provided."),
    }
}
