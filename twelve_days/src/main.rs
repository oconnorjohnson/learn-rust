fn main() {
    let mut lyrics = String::from("");
    let mut curr_verse = String::from("");
    let mut curr_pres = String::from("");
    let mut rev_pres = String::from("");
    let on_the: &str = "On the ";
    let day_of: &str = " day of Christmas, my true love gave to me: ";
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelve",
    ];
    let presents = [
        "a partridge in a pair tree.",
        "Two turle doves, and ",
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
    for (index, element) in days.iter().enumerate() {
        curr_verse.clear();
        curr_verse.push_str(&on_the);
        let curr_day = element;
        curr_verse.push_str(&curr_day);
        curr_verse.push_str(&day_of);
        if index == 0 {
            curr_pres = presents[index].to_string();
            curr_verse.push_str(&curr_pres);
        } else {
            for &item in presents.iter().take(index + 1) {
                rev_pres.insert_str(0, item);
            }
        }
        curr_verse.push_str(&rev_pres);
        curr_verse.push_str("\n");
        lyrics.push_str(&curr_verse);
    }
    println!("{lyrics}");
}
