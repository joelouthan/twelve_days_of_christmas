use std::thread;
use std::time::Duration;

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me:", days[i]);
        thread::sleep(Duration::from_secs(1));
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                println!("and {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
            thread::sleep(Duration::from_secs(1));
        }
        println!();
        thread::sleep(Duration::from_secs(1));
    }
}
