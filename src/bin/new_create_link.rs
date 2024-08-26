// 用Rust创建一个链表

// 步骤是啥

// 一、链表的模型

// 二、创建链表的函数


// 三、递归把每个链表都连接起来

// 注意点：1、enum的拼法。2、每个语句都要用分号

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    // 创建一个空的链表节点
    fn new() -> List {
        Nil
    }
    // 在链表头部添加元素，并返回List
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    // 返回链表的长度
    fn len(&self) -> u32 {
        // &self 是借用类型 &List，*self 就是List类型
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    // 字符串打印
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("head: {}, tail: {}", head, tail.stringify()),
            Nil => format!("None")
        }
    }
}

fn main() {
    let mut list = List::new();
    
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    println!("list is {}", list.len());
    println!("list is {}", list.stringify());
}