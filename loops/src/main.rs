fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    let mut numba = 3;
    while numba != 0 {
        println!("{numba}");
        numba -= 1;
    }
    println!("Lift off!");
}
