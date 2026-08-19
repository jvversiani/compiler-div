// Rosetta Code task: The Twelve Days of Christmas
// Source: https://rosettacode.org/wiki/The_Twelve_Days_of_Christmas#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// On the first day of Christmas,
// My true love gave to me:
// A Partridge in a Pear Tree
// 
// On the second day of Christmas,
// My true love gave to me:
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the third day of Christmas,
// My true love gave to me:
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the fourth day of Christmas,
// My true love gave to me:
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the fifth day of Christmas,
// My true love gave to me:
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the sixth day of Christmas,
// My true love gave to me:
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the seventh day of Christmas,
// My true love gave to me:
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the eighth day of Christmas,
// My true love gave to me:
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the ninth day of Christmas,
// My true love gave to me:
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the tenth day of Christmas,
// My true love gave to me:
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the eleventh day of Christmas,
// My true love gave to me:
// Eleven Pipers Piping
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
// 
// On the twelfth day of Christmas,
// My true love gave to me:
// Twelve Drummers Drumming
// Eleven Pipers Piping
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves and
// A Partridge in a Pear Tree
//
// =======================

fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
                "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = ["A Partridge in a Pear Tree",
                 "Two Turtle Doves and",
                 "Three French Hens",
                 "Four Calling Birds",
                 "Five Golden Rings",
                 "Six Geese a Laying",
                 "Seven Swans a Swimming",
                 "Eight Maids a Milking",
                 "Nine Ladies Dancing",
                 "Ten Lords a Leaping",
                 "Eleven Pipers Piping",
                 "Twelve Drummers Drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love gave to me:");

        for j in (0..i + 1).rev() {
            println!("{}", gifts[j]);
        }
        println!()
    }
}
