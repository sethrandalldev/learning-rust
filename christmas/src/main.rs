fn main() {
    const CHRISTMAS_DAYS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let mut i: usize = 0;
    while i < 12 {
        println!("On the twelfth day of Christmas, my true love sent to me");
        for j in (0..=i).rev() {
            println!("{}", CHRISTMAS_DAYS[j]);
        }
        println!("");
        i += 1;
    }
}
