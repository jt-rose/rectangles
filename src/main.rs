fn main() {
    let rectangle = Rectangle::new(
        30,
        50
    );

    println!("Current Rectangle: {:#?}", rectangle);
    println!("The rectangle has an area of {}", rectangle.get_area());
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }

}