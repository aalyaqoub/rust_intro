
fn twelve_day_christmas(day: usize, days_gift: [(&str, &str); 12]) {
    if day == 0 {
        println!("And {}\n", days_gift[day].1.to_lowercase());
    } else {
        println!("{}", days_gift[day].1);
        twelve_day_christmas(day-1, days_gift)
    }
}

fn main() {
    let days_gift: [(&str, &str); 12] = [("first", "A partridge in a pear tree"),
                                          ("second", "Two turtle doves"),
                                          ("third", "Three French hens"),
                                          ("fourth", "Four calling birds"),
                                          ("fifth", "Five golden rings"),
                                          ("sixth", "Six geese a-laying"),
                                          ("seventh", "Seven swans a-swimming"),
                                          ("eighth", "Eight maids a-milking"),
                                          ("ninth", "Nine ladies dancing"),
                                          ("tenth", "Ten lords a-leaping"),
                                          ("eleventh", "Eleven pipers piping"),
                                          ("twelfth", "Twelve drummers drumming")];
    
    for (index, day_gift) in days_gift.iter().enumerate() {
        if index == 0 {
            println!("On the {} day of Christmas my true love sent to me\n{}\n", day_gift.0, day_gift.1);
        } else {
            println!("On the {} day of Christmas my true love sent to me", day_gift.0);
            twelve_day_christmas(index, days_gift);
        }
        
    }

}