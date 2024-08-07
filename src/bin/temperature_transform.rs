// 练习题一：在华氏温度和摄氏度之间转换温度。
use std::io;
fn main() {
    // 让用户选择是华氏温度还是摄氏温度
    println!("1代表华氏温度，2代表摄氏温度");

    let mut temperatrue_type = String::new();

    loop {
        println!("请选择你输入的温度格式")

        io::stdin().read_line(&mut temperatrue_type).expect("读取失败");
    }

    let mut temperature = String::new();
    // 然后让用户输入温度

    println!("请输入你的温度值");

    io::stdin().read_line(&mut temperature).expect("读取失败");

    // 类型转换，将用户输入的字符串转换成数字

    let temperature: u32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("输入的不是温度值，请重新开始")
    }

    // 如果是华氏温度就选择华氏温度转换函数
    let x = if temperatrue_type == '1' {transform_c_f(temperature)} else {transform_f_c(temperature)};
    // 如果是摄氏温度就选择摄氏温度转换函数
    println!("transform result is {}", x);
}


fn transform_f_c(temperature: u32) -> u32 {
    // ℃ = (°F - 32) × 1.8
    (temperature - 32) * 1.8;
}

fn transform_c_f(temperature: u32) -> u32 {
    // f = c / 1.8 + 32
    temperature / 1.8 + 32;
}