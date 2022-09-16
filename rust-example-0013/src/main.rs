#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
// use peroxide::fuga::*;
use std::env;

fn main() {



    let args: Vec<String> = env::args().collect::<Vec<_>>();

    // println!("{:?}", args);
    // for arg in &args {
    //     println!("{:?}", arg);
    // }

    // let x1 = &args[0];
    let x2 = &args[1];
    // println!("{:?}{:?}", x1, x2);

    let x3 = x2.parse::<f64>().unwrap();
    // let x3 = x2.parse::<Vec<f64>>().unwrap();
    println!("{}",x3/100.);



    
}


