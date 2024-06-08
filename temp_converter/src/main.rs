use std::io;

fn main() {
    println!("Input a number to convert from farenheit to celsius.");
    let mut f_temp = String::new();
    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");
    let f_temp: f32 = f_temp
        .trim()
        .parse()
        .expect("input can not be parsed to a number");
    println!("Converting {f_temp} degrees farenheit to celsius.");
    let c_temp: f32 = (f_temp - 32.0) * (5.0 / 9.0);
    println!("In celsius, the temperature is {c_temp} degrees.");
}

// to do:
// re-write main to take user input of celsius vs farenheit and run the necessary conversion
// add a loop for repeated conversions and exit on non number input (e.g 'quit')
