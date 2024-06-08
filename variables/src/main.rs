fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 5;
    println!("The new value of x is: {x}");
    {
        let x = x * 2;
        println!("The new inner scope value of x is: {x}");
    }
    let mut y = 5;
    println!("The value of mut y: {y}");
    y = y + 50;
    println!("The new value of mut y: {y}");
    let sum = 5 + 10;
    println!("sum: {sum}");
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    let product = 4 * 20;
    println!("product: {product}");
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 3;
    println!("truncated: {truncated}");
    let remainder = 43 % 5;
    println!("remainder: {remainder}");
    let t = true;
    let f: bool = false;
    println!("t is: {t} and f is: {f}");
    // char file types
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    // create a tuplet:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // use pattern matching to destructure the tuplet:
    let (h, j, k) = tup;
    println!("The values are h: {h}, j: {j}, k: {k}");
    // we can also access a tuple element directly as so:
    let z: (i32, f64, u8, char, &str) = (500, 6.4, 1, 'a', "asd");
    let five_hundred = z.0;
    let six_point_four = z.1;
    let one = z.2;
    let char = z.3;
    let string = z.4;
    println!("{five_hundred}, {six_point_four}, {one}, {char}, {string}");
    // create an array:
    let arr = [1, 2, 3, 4, 5];
    // unlike a tuple, every element in an array must have the same type
    // like a tuple, arrays have a fixed length
    // arrays are useful when you want your data allocated on the stack rather than the heap
    // arrays also useful when you want to ensure you always have a fixed number of elements
    // not as flexible as vector type
    // a vector is a similar collection type provided by the std library
    // a vector IS allowed to grow in shape and size.
    // when unsure between an array and vector, chances are you should use a vector
    // more on vectors in chapter 8
    // arrays are more useful when you know the number of elements will not need to change.
    // for example if you're using the names of the 12 months in a program these should be stored in an array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // write an array's type using square brackets
    // put first the type of elements, a semicolon, then the number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // you can also initialize an array to contain the same value for each element
    // specify the original value, followed by a semicolo, and then the length:
    let b = [3, 5];
    // the above is the same as writing:
    let b: [i8; 5] = [3, 3, 3, 3, 3];
    // now access indexes from the array like so:
    let u = [1, 2, 3, 4, 5];
    let first = u[0];
    let second = u[1];
    println!("first: {first}, second: {second}");
}
