#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
mod add;

pub use add::add;

// mod coeff;

fn main() {
    let vx1 = vec![1, 2, 3];
    let vx2 = vec![4, 5, 6];
    let mx1 = py_matrix(vec![vx1]);
    let mx2 = py_matrix(vec![vx2]);

    // 迭代器-高级for循环-使用迭代器处理元素序列
    let v1_iter
    = mx1.row(0)   
    .iter()    // 创建一个迭代器(只能对vector或者list取迭代器)
    .enumerate()  //使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个元组中。
    .map(|(idx,value)| {add(*value, mx2.row(0)[idx])}) //调用迭代器适配器 map 来创建一个新迭代器
    .map(|x1| vec![x1]) 
    .collect::<Vec<_>>(); //使用的 collect 方法,消费迭代器并将结果收集到一个数据结构中

    println!("{:?}",v1_iter);

}
