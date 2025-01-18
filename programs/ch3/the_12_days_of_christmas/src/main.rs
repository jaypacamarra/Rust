fn main() {
    let presents = [ 
        "A partridge in a pear tree!",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling bird,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "sevent",
        "eighth",
        "ninth",
        "tenth",
        "elenenth",
        "twelvth",
    ];

    let mut day = 1;
    loop {
        println!("On the {} day of Christmas,", days[day-1]); 
        println!("my true love gave to me"); 

        for present in (0..day).rev() {
            if day == 1 {
                println!("{}",presents[present]);
            }
            else {
                if present == 0  {
                    println!("And {}",presents[present]);
                }
                else {
                    println!("{}",presents[present]);
                }
            }
        }
        println!("");

        day = day + 1;
        
        if day > 12 {
            break
        }
    }
}
