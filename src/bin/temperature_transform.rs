use std::io;
// 这题的要点：
// 1. rust很多地方都有分号。
// 2. 对rust的类型转换还不太熟需要在学。
// 3. 变量应该存在块级作用域，后面要注意，这边的loop就只能写到全局去。

fn main() {
    loop {
        println!("1代表华氏温度，2代表摄氏温度");
        
        let mut temperatrue_type = String::new();
        let mut temperature = String::new();

        println!("请选择你输入的温度格式");
        io::stdin().read_line(&mut temperatrue_type).expect("读取失败");
        temperatrue_type = temperatrue_type.trim().to_string();

        if temperatrue_type != "1" && temperatrue_type != "2" {
            println!("输入错误，请重新选择温度格式");
            continue;
        }

        println!("请输入你的温度值");
        io::stdin().read_line(&mut temperature).expect("读取失败");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的不是温度值，请重新开始");
                continue;
            }
        };

        let x = if temperatrue_type == "1" { transform_c_f(temperature) } else { transform_f_c(temperature) };
        println!("转换结果为 {}", x);
        break;
    }
}

fn transform_f_c(temperature: u32) -> u32 {
    (temperature - 32) * 5 / 9
}

fn transform_c_f(temperature: u32) -> u32 {
    temperature * 9 / 5 + 32
}