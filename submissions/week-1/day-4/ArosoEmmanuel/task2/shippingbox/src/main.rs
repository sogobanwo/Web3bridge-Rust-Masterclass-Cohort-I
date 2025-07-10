fn main() {
    println!("Hello, world!");
}


struct ShippingBox {
    dimensions: u32,
    weight: f32,
    color: Color,
}

enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    Black,
    White,
}

impl ShippingBox {

    fn new(dimensions: u32, weight: f32, color: Color) -> Self {
        ShippingBox {
            dimensions,
            weight,
            color,
        }
    }

    // ToDo: apply calculations for the dimensions
    fn calculate_volume(&self) -> u32 {
        // Assuming dimensions is the volume in cm³ for simplicity
        self.dimensions
    }

    fn display_characteristics(box: &ShippingBox) {
        let box = box; // shadowing the parameter to avoid confusion
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {} cm³", box.dimensions);
        println!("Weight: {} kg", box.weight);
        println!("Color: {}", box.display_color());
    }

    fn display_color(box: &ShippingBox) -> &str {
        match box.color {
            Color::Red => "Red",
            Color::Orange => "Orange",
            Color::Yellow => "Yellow",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Indigo => "Indigo",
            Color::Violet => "Violet",
            Color::Black => "Black",
            Color::White => "White",
        }
    }
}   
