fn main() {
    let number = 3;
    if number > 5 {
        println!("Condition was true!")
    } else {
        println!("Condition was false :(")
    }
    if number != 1 {
        println!("Condition was not equal to one.")
    }
    if_else();
    if_in_let();
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
