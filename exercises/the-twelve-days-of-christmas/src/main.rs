fn main() {
    let days = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings!",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            get_position(i as u32 + 1)
        );
        for j in (0..i + 1).rev() {
            println!(
                "{}",
                format!(" {}{}", if i > 0 && j == 0 { "and " } else { "" }, days[j])
            );
        }
    }
}

fn get_position(n: u32) -> String {
    if n % 10 == 1 {
        n.to_string() + "st"
    } else if n % 10 == 2 {
        n.to_string() + "nd"
    } else if n % 10 == 3 {
        n.to_string() + "rd"
    } else {
        n.to_string() + "th"
    }
}
