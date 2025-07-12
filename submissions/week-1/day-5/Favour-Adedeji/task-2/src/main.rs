enum Book {
    Fiction {
        title: String,
        author: String,
        price: f32,
    },
    Magazine {
        title: String,
        author: String,
        price: f32,
    },
    SciFi {
        title: String,
        price: f32,
    },
}

fn main() {
    let fiction = Book::Fiction {
        title: String::from("The Great Adventure"),
        author: String::from("Jane Doe"),
        price: 15.50,
    };

    let magazine = Book::Magazine {
        title: String::from("Tech Weekly"),
        author: String::from("Rick Smith"),
        price: 149.99,
    };

    let scifi = Book::SciFi {
        title: String::from("Galactic Wars"),
        price: 89.99,
    };

    let books = vec![fiction, magazine, scifi];

    for book in &books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction Book '{}' by {} is sold for ${}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine Book '{}' by {} is sold for ${}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("SciFi Book '{}' is sold for ${:.2}", title, price);
            }
        }
    }
}
