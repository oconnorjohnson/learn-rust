fn main() {
    println!("Let's calculate the nth fibonacci number.");
    let steps = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut prev: u16 = 0;
    let mut current: u16 = 1;
    for element in steps {
        if element == 1 {
            println!("{prev}");
        } else if element == 2 {
            println!("{current}");
        } else {
            let next: u16 = current + prev;
            prev = current;
            current = next;
            println!("{current}")
        }
    }
}
