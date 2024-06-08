fn main() {
    println!("started main");
    another_function(-5, 23.43);
}

fn another_function(x: i8, y: f32) {
    println!("started another_function with arguments of x: {x} and y: {y}");
}
