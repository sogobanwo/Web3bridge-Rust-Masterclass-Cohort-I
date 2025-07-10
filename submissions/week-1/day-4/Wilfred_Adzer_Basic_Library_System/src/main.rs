// --- Task 1: Library Item Management ---
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}

// --- Task 2: Shipping Box with `impl` ---

enum Color {
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: Color) -> Self {
        Self {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_details(&self) {
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
        }
    }
}


fn main() {
    println!("--- Task 1: Library Item ---");
    let item = LibraryItem {
        quantity: 10,
        id: 1234,
        item_type: ItemType::Book,
    };

    display_quantity(&item);
    display_id(&item);
    display_type(&item);

    println!("\n--- Task 2: Shipping Box ---");
    let box1 = ShippingBox::new(10.0, 5.0, 3.0, 2.5, Color::Green);
    box1.print_details();
}
