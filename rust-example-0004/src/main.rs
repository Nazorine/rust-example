#![allow(non_snake_case)]
// #[macro_use]

extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    // println!("Hello, world!");

    let vx1 = [1, 2, 3];
    let vx2 = [4, 5, 6];
    // let vx3 = vx1 + vx2; // 不能相加
    println!("{:?}", vx1);
    println!("{:?}", vx2);

    let vx4 = vec![1, 2, 3];
    let vx5 = vec![4, 5, 6];
    // let vx6 = vx4 + vx5;  // 不能相加
    println!("{:?}", vx4);
    println!("{:?}", vx5);

    //--- Scalar和vector是最重要的基本概念 ---//
    //--- 一维是零维的嵌套、多维是一维的嵌套 ---//
    let vx6 = vec![vec![1, 2, 3]];
    let vx7 = vec![vec![4, 5, 6]];
    let mx6 = py_matrix(vx6);
    let mx7 = py_matrix(vx7);
    mx6.print();
    mx7.print();

    let mx8 = &mx6 + &mx7; // 矩阵加法
    mx8.print();

    let mx9 = &mx6 * &mx7.transpose(); // 矩阵乘法：需满足i*m m*j
    mx9.print();

    let vx10 = vec![vec![1, 2, 3], vec![2, 5, 4], vec![7, 8, 2]];
    let mx10 = py_matrix(vx10);
    mx10.print();
    mx10.det().print(); // 矩阵的行列式
    mx10.inv().print(); // 矩阵的逆
    mx10.transpose().print(); // 矩阵的转置
    mx10.diag().print(); // 矩阵的主对角元素
    mx10.col.print(); // 矩阵的列数
    mx10.row.print(); // 矩阵的行数
    
    let mx10_col = mx10.col(0); // 矩阵的某一列，取出后转化为vector
    let mx10_row = mx10.row(0); // 矩阵的某一行，取出后转化为vector

    mx10_col.print();
    mx10_row.print();



}
