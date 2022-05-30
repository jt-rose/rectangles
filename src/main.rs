fn main() {
    let rectangle = Rectangle::new(
        30,
        50
    );

    // using debug trait with pretty printing
    println!("Current Rectangle: {:#?}", rectangle);

    // using the debug macro
    dbg!(&rectangle);

    println!("The rectangle has an area of {}", rectangle.get_area());
}

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