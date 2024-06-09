fn main() {
    let mut lyrics = String::from("");
    let mut curr_verse = String::from("");
    let on_the: &str = "On the";
    let _day_of: &str = "day of Christmas, my true love gave to me:";
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelve",
    ];
    let _presents = [
        "A partridge in a pair tree.",
        "Two turle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];
    for (index, element) in days.iter().enumerate() {
        curr_verse.push_str(&on_the);
        let curr_day = days[index];
        curr_verse.push_str(&curr_day);
        println!("{curr_verse}");
        println!("{element}");
        // we need to loop through the days array, printing the day between the on_the and day_of
        // on each iteration if index > 0, we add all previous indices of 'presents' after the current is printed
        // added in reverse order. This loop adds lyrics to the end of a string element (&mut lyrics)
        // so each loop adds `on_the`, the current element in `days`, `day_of`, then the indexes of `presents` starting
        // from our current index of `days`, and going backwards through the elements
        // then at the end of the loop we append `curr_verse` to the end of `lyrics`
        lyrics.push_str(&curr_verse);
    }
    println!("{lyrics}");
}
