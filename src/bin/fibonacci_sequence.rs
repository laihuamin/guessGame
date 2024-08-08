// 练习题二：生成 n 阶斐波那契数列。
use std::io;

fn main() {
    loop {
        // 用户输入一个数字
        let mut number_str = String::new();
        println!("请输入一个数字");

        io::stdin().read_line(&mut number_str).expect("读取失败");

        let int_num: i32 = match number_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的不是一个整数数字，请重新在输入");
                continue;
            }
        };

        // 然后计算这个数字对应的斐波那契数列
        let x = calc_fibonacci(int_num);
        println!("斐波那契数列计算结果：{}", x);
        break;
    }
    
}
// 是个递归
fn calc_fibonacci(num: i32) -> i32 {
    if num <= 0 { return 0 };
    if num == 1 { return 1 };
    return calc_fibonacci(num - 1) + calc_fibonacci(num - 2);
}