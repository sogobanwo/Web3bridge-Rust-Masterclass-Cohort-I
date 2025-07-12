// Define the Book enum with associated data for each variant
#[derive(Debug)]
enum Book {
    Fiction {
        title: String,
        author: String,
        price: f64,
    },
    Magazine {
        title: String,
        author: String,
        price: f64,
    },
    SciFi {
        title: String,
        price: f64,
    },
}

fn main() {
    // Create instances of each book type
    let books = vec![
        Book::Fiction {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            price: 20500.0,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Contributors".to_string(),
            price: 9000.0,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 25600.0,
        },
        Book::Fiction {
            title: "Pride and Prejudice".to_string(),
            author: "Jane Austen".to_string(),
            price: 16800.0,
        },
        Book::Magazine {
            title: "Scientific American".to_string(),
            author: "Editorial Team".to_string(),
            price: 12800.0,
        },
        Book::SciFi {
            title: "The Martian".to_string(),
            price: 22400.0,
        },
    ];

    println!(" Book Inventory:");
    println!("==================");

    // Iterate through the vector and use match to print book information
    for (index, book) in books.iter().enumerate() {
        print!("{}. ", index + 1);

        match book {
            Book::Fiction {
                title,
                author,
                price,
            } => {
                println!("Fiction: \"{}\" by {} - #{:.2}", title, author, price);
            }
            Book::Magazine {
                title,
                author,
                price,
            } => {
                println!("Magazine: \"{}\" by {} - #{:.2}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("Sci-Fi: \"{}\" - #{:.2}", title, price);
            }
        }
    }