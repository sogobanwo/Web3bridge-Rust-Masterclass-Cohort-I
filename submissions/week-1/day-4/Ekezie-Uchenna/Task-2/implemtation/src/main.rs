


#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Orange,
}

#[derive(Debug)]
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

impl ShippingBox {
    fn ship(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }
        
    fn print_characteristics(&self) {
       
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        print!("Color: ");
        match self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Blue => println!("Blue"),
            BoxColor::Green => println!("Green"),
            BoxColor::Orange => println!("Orange"),
        }
    }
}

fn main() {
    
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Blue);


    box1.print_characteristics();
}

