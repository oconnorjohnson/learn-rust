struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rec1 = Rectangle {
        height: 25,
        width: 90,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_is(&rec1)
    );
}

fn area_is(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
