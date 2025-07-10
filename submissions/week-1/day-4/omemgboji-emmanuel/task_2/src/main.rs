
enum Color(
    White(u8,u8,u8);
    Black(u8,u8,u8);
)

struct ShippingBox{
    dimensions: u32,
    weight: u32,
    color: Color,
}


impl ShippingBox{
    fn new_box(dimen: (u32, u32), wght: u32, color: Color) -> Self{
        let dimensions = dimen.0 * dimen.1;

        Self{
            dimensions,
            weight: wght,
            color
        }
    }

    fn display_dim(&self) ->u32{
        self.dimensions
    }

    fn display_wght(&self) -> u32{
        self.weight
    }

    fn display_color(&self) ->Color{
        self.color
    }
}
fn main() {
    
    ShippingBox::new_box();

    println!("Dimensions: {}", ShippingBox::display_dim());
}
