fn main() {

    

}


#[derive(Debug)]
enum color_type{
    red, blue, yellow
}


#[derive(Debug)]
struct characteristics {
    dimemsion: i32,
    weight: i32,
    color: color_type,
}

impl characteristics {
    
    fn create_box(&self) {

        let new_box = &self;

        new_box

    }

    fn print_char(){

        println!("This is charteristics {:?}", &self);

    }
}