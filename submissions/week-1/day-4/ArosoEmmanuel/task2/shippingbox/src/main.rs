fn main() {

    let box1 = ShippingBox::new(100, 5.0, Color::Blue);
    let box2 = ShippingBox::new(200, 10.0, Color::
        Red);

    ShippingBox::display_characteristics(&box2);

    // Displaying the volume of box1
    // println!("Box 1 Volume: {} cm³", box1.calculate_volume());
}


struct ShippingBox {
    dimensions: u32,
    weight: f32,
    color: Color,
}

#[derive(Debug)]
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

    fn display_characteristics(whole_box: &ShippingBox) {
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {} cm³", whole_box.calculate_volume());
        println!("Weight: {} kg", whole_box.weight);
        whole_box.display_color(whole_box);
    }

    fn display_color(&self, color_box: &ShippingBox) {
        match color_box.color {
            Color::Red => println!("Color: Red"),
            Color::Orange => println!("Color: Orange"),
            Color::Yellow => println!("Color: Yellow"),
            Color::Green => println!("Color: Green"),
            Color::Blue => println!("Color: Blue"),
            Color::Indigo => println!("Color: Indigo"),
            Color::Violet => println!("Color: Violet"),
            Color::Black => println!("Color: Black"),
            Color::White => println!("Color: White"),
        }
    }
}   
