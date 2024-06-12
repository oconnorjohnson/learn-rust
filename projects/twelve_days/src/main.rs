fn main() {
    let on_the: &str = "On the ";
    let day_of: &str = " day of Christmas, my true love gave to me: ";
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelve",
    ];
    let presents = [
        "a partridge in a pair tree.",
        "Two turtle doves, and ",
        "Three French hens, ",
        "Four calling birds, ",
        "Five golden rings, ",
        "Six geese a-laying, ",
        "Seven swans a-swimming, ",
        "Eight maids a-milking, ",
        "Nine ladies dancing, ",
        "Ten lords a-leaping, ",
        "Eleven pipers piping, ",
        "Twelve drummers drumming, ",
    ];
    let mut lyrics = String::new();
    for day in (0..12) {
        let mut verse = String::new();
        verse.push_str(on_the);
        verse.push_str(days[day]);
        verse.push_str(day_of);
        for gift in presents.iter().take(day + 1).rev() {
            verse.push_str(gift);
        }
        verse.push('\n');
        lyrics.push_str(&verse);
    }
    println!("{lyrics}");
}
