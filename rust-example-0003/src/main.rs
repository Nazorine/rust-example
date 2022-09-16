#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;


fn main() {
    // println!("Hello, world!");

    let x1 = 1;
    let x2 = x1;  // 简单数据类型存在栈上，自动copy
    let x3 = x1;
    x1.print();
    x2.print();
    x3.print();

    let vx1 = [1,2,3]; 
    let vx2 = vx1;  // 简单数据类型(scalar、list)存在栈上，自动copy
    println!("{:?}",vx1);
    println!("{:?}",vx2);

    let vx3 = vec![1,2,3];  // 复杂数类型(vector)
    let vx4 = vx3;          // 使用后所有权移交至新的变量
    // println!("{:?}",vx3);          // 这里不能再使用vx3 
    println!("{:?}",vx4);

    let vx5 = vec![4,5,6];
    let vx6 = &vx5; 
    // let vx7 = *vx6;
    vx5.print();
    vx6.print();
    println!("{:?}",*vx6);

}
