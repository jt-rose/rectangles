fn main() {
    let rectangle = Rectangle{
        width: 30,
        height: 50
    };

    let rectangle_area = area(&rectangle);

    println!("The rectangle has an area of {}", rectangle_area);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32
}