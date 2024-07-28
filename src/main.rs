// std代表标准 io是标准库中的一个库
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("please input your guess");
        // let用于生命一个变量 mut代表可变与不可变
        let mut guess = String::new();

        // io模块调用stdin函数，允许我们处理用户输入
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //  rust允许用一个新的值来遮蔽之前的值。
        // 这里做了一个类型转换，从string转换为一个number类型。
        // 做处理，如果用户输入非数字，则继续让他猜
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                // 如果猜对了就推出
                break;
            }
        }
    }
    
}
