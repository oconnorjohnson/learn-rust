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
        // right now each verse is getting every day
        // we want each verse (each run of the loop) to get a single on_the, day, and of_the
        curr_verse.push_str(&on_the);
        let curr_day = days[index];
        curr_verse.push_str(&curr_day);

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

// current output:
// Running `target/debug/twelve_days`
// first
// second
// third
// fourth
// fifth
// sixth
// seventh
// eighth
// ninth
// tenth
// eleventh
// twelve
// On thefirstOn thefirstOn thesecondOn thefirstOn thesecondOn thethirdOn thefirstOn thesecondOn thethirdOn thefourthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn theeighthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn theeighthOn theninthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn theeighthOn theninthOn thetenthOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn theeighthOn theninthOn thetenthOn theeleventhOn thefirstOn thesecondOn thethirdOn thefourthOn thefifthOn thesixthOn theseventhOn theeighthOn theninthOn thetenthOn theeleventhOn thetwelve
