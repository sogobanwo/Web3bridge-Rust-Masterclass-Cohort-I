fn main() {
    println!("Hello, world!");
}


impl ShippingBox{

    fn new_box() -> Self {
        Self {
            
        }
    }
}



#[derive(Debug)]
struct ShippingBox {
    width: i32,
    dimension: Dimension,
    color: BoxColor,
}


#[derive(Debug)]
struct Dimension {
    length: i32,
    width: i32,
    height: i32,
}


#[derive(Debug, Copy, Clone)]
enum BoxColor {
    Red,
    Blue,
    Green,
}