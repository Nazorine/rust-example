#![allow(non_snake_case)]
#![allow(unused)]
use std::str::from_utf8;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string_pretty;

fn main() {
    

    // json字符串
    // r#" "#   r表示使用转义字符将回车等特殊字符也表示出
    let json = r#"
    {
    "name": "liuyang",
    "age": 25,
    "email": "yliunssept@163.com"
    }"#;

    // 将json字符串String转换为rust自带的Value类型Struct
    let str1: serde_json::Value = serde_json::from_str(json).unwrap();
    println!("name = {}", str1["name"]);
    println!("age = {}", str1["age"]);
    println!("eamil = {}", str1["email"]);
    println!("{:?}",json);



}
