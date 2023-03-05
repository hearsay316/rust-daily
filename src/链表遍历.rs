#[derive(Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

use std::collections::LinkedList;
use std::mem::forget;
use std::ops::Add;

fn main() {
    println!("你好");
    let mut  a = ListNode::new(1);
    a.next = Option::Some(Box::new(ListNode::new(2)));

    let list = LinkedList::from([1, 2, 3]);
    // println!("{}",list.len());
    let mut iter = list.iter();
    let mut  arr  = vec![];
    for i in 0..list.len() {
        match iter.next() {
            Some(i)=>{
                arr.push(i);
                println!("{}",i);
            }
            _ => {}
        }
    }
    println!("{:?}",arr);
    // println!("{:?}",a.next.unwrap().next.unwrap());
    // loop {
    //     match a {
    //         // 如果 `optional` 解构成功，就执行下面语句块。
    //         Some(ref i) => {
    //             println!("{:?}",i);
    //             // ^ 需要三层缩进！
    //             match a.next {
    //                 // 如果 `optional` 解构成功，就执行下面语句块。
    //                 Some(ref i) => {
    //                     println!("{:?}",i);
    //                     // ^ 需要三层缩进！
    //                     a  = *a.next.unwrap();
    //                 },
    //                 // 当解构失败时退出循环：
    //                 _ => { break; }
    //                 // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
    //             }
    //         },
    //         // 当解构失败时退出循环：
    //         _ => { break; }
    //         // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
    //     }
    // }
greet(&String::from("XHgdvedfv"));
    let name = "name";
    let name2 = String::from("XHgdvedfv");
    let name3:&String = &name2;
    greet(name3);
    let five = Box::new(5).add(1);
    println!("{five}");
}

fn greet(S:&str){
    println!("{S}");
}
