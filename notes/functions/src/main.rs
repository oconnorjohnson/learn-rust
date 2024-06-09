fn main() {
    println!("started main");
    another_function(-5, 23.43);
    let y = {
        let x = 3;
        x + 5
    };
    println!("y is equal to {y}");
    let z = a_third_function(69418);
    println!("z is equal to {z}");
}

fn another_function(x: i8, y: f32) {
    println!("started another_function with arguments of x: {x} and y: {y}");
}

fn a_third_function(x: u32) -> u32 {
    println!("started a_third_function with an argument of x: {x}");
    x + 2
}
