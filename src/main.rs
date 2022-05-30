fn main() {
    let width: u32 = 10;
    let height: u32 = 15;

    let rectangle_area = area(width, height);

    println!("The rectangle has an area of {}", rectangle_area);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}