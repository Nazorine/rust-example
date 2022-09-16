#![allow(non_snake_case)]
#![allow(unused)]
use std::str::from_utf8;
use serde::Serialize;
use serde::Deserialize;

fn main() {
    
    let n_layer = 8;
    let L = vec![1e15, 1e15, 1e15, 1e15, 1e15, 1e15, 1e15];
    let Emm = vec![vec![9855., 12000., 11000., 9000., 13000., 13000., 720., 40.8]];
    let mu = vec![0.25, 0.25, 0.25, 0.40, 0.25, 0.25, 0.35, 0.40];
    let h = vec![0.04, 0.06, 0.08, 0.1, 0.18, 0.18, 0.20];
    let P = 0.7;
    let Q = 0.;
    let a = 0.1065;
    let p = vec![0.7];
    let q = vec![0.];
    let xx_ = vec![0., 0.];
    let yy_ = vec![-0.15975, 0.15975];
    let xx = vec![0.001, 0.001, 0.001, 0.001];
    let yy = vec![0.001, 0.026625, 0.054, 0.15975];
    let n_d = 50;
    let upper_h = 400.;
    let n_i = 200;
    let n_series = 1;
    let zz = vec![0., 0.01, 0.04, 0.04, 0.28, 0.28];
    let NN = vec![1, 1, 1, 2, 4, 5];

    let input_python = InputData {
        n_layer,
        L,
        Emm,
        mu,
        h,
        P,
        Q,
        a,
        p,
        q,
        xx_,
        yy_,
        n_d,
        upper_h,
        n_i,
        n_series,
        xx,
        yy,
        zz,
        NN
    };
    println!("{:?}", input_python);

    // rust使用serde_json序列化结构体
    let str1 = serde_json::to_string(&input_python).unwrap();
    println!("{:?}\n", str1);

    // rust将json字符串String转换为字节数组Vec<u8>
    let byte1 = str1.into_bytes();
    println!("{:?}\n", byte1);

    // rust将字节数组Vec<u8>转换为十六进制字节串String
    let hex_str = hex::encode(byte1);
    println!("{:?}\n",hex_str);


}

#[derive(Serialize, Deserialize,Debug)]
struct InputData {
    n_layer: i32,
    L: Vec<f64>,
    Emm: Vec<Vec<f64>>,
    mu: Vec<f64>,
    h: Vec<f64>,
    P: f64,
    Q: f64,
    a: f64,
    p: Vec<f64>,
    q: Vec<f64>,
    xx_: Vec<f64>,
    yy_: Vec<f64>,
    n_d: i32,
    upper_h: f64,
    n_i: i32,
    n_series: i32,
    xx: Vec<f64>,
    yy: Vec<f64>,
    zz: Vec<f64>,
    NN: Vec<i32>,
}