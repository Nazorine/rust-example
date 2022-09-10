#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    println!("Hello, world!");

    let x1 = 1;
    let x2 = 2.0;
    let x3 = "hello";

    x1.print();
    x2.print();
    println!("{:?}",x3);

    let vx1 = [1,2,3];
    let vx2 = vec![1,2,3];
    let vx3 = vec![vec![1,2,3]];
    let vx4 = vec![vec![vec![1,2,3]]];

    println!("{:?}",vx1);
    println!("{:?}",vx2);
    println!("{:?}",vx3);
    println!("{:?}",vx4);


}
