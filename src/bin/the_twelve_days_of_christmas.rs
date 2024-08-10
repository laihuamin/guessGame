// 练习题三：打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，利用歌曲中的重复部分（编写循环）。
// 歌词部分
// On the first day of Christmas,
// my true love gave to me
// A partridge in a pear tree.

// On the second day of Christmas,
// my true love gave to me
// Two turtle doves,
// And a partridge in a pear tree.

// On the third day of Christmas,
// my true love gave to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the fourth day of Christmas,
// my true love gave to me
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the fifth day of Christmas,
// my true love gave to me
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the sixth day of Christmas,
// my true love gave to me
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the seventh day of Christmas,
// my true love gave to me
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the eighth day of Christmas,
// my true love gave to me
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the ninth day of Christmas,
// my true love gave to me
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the tenth day of Christmas,
// my true love gave to me
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the eleventh day of Christmas,
// my true love gave to me
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!

fn main() {
    const DAY_TIME: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    const DAY_ACTION: [&str; 12] = ["partridge in a pear tree", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];
    let mut num: usize = 1;

    while num <= 12 {
        println!("On the {} day of Christmas", DAY_TIME[num - 1]);
        println!("my true love gave to me");

        // 把做的事情循环一下
        for i in (0..num).rev() {
            if i == 0 {
                let tmp_str: &str = if num == 1 { "A" } else { "and a" };
                println!("{} {}", tmp_str, DAY_ACTION[i]);
            } else {
                println!("{}", DAY_ACTION[i]);
            }
        }
        println!(" ");
        
        num += 1;
    }
}