#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> usize {
    println!("User Mode Program: HELLO WORLD!");
    println!("下面是用户程序的thread情况 \n 和程序主体：循环输出");
    for i in 0..10000000{
        if i%1000000 == 0 {
            println!("HELLO WORLD {} times",i/1000000);
        }
    }
    0
}
