fn main() {
    println!("Let's calculate the nth fibonacci number.");
    let steps = [1, 2, 3, 4, 5, 6, 7];
    let mut prev: u32 = 0;
    let mut current: u32 = 1;
    for element in steps {
        if element == 1 {
            println!("{prev}");
        } else if element == 2 {
            println!("{current}");
        } else {
            let next = current + prev;
            prev = current;
            current = next;
            println!("{current}")
        }
    }
}
