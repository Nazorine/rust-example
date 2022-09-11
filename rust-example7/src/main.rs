#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
//
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::to_string_pretty;


fn main() -> Result<(), Box<dyn Error>>  {

    // let x1_ = 1;
    // let vx1_ = vec![1.0,2.0,3.0];
    // let vvx1_ = vec![vec![1.0,2.0,3.0]];

    // let Data_Write_To_Input = WriteToInput {
    //     x1:&x1_,
    //     vx1: &vx1_,
    //     vvx1: &vvx1_,
    // };
    // let Data_Out_Write_To_Input = to_string_pretty::<WriteToInput>(&Data_Write_To_Input)?;
    // let mut Data_In_Write_To_Input = BufWriter::new(File::create("input.json")?);
    // write!(&mut Data_In_Write_To_Input, "{}", Data_Out_Write_To_Input)?;

    // println!("{:?}",Data_Write_To_Input);


    /////从input.json文件读取输入数据
    let Data_Read_From_Input_File = BufReader::new(File::open("input.json")?);
    let InputData {
        x1,
        vx1,
        vvx1,
    } = from_reader::<_, InputData>(Data_Read_From_Input_File)?;

    println!("{}",x1);

    

    let Data_Write_To_Output = OutputData {
        x1:&x1,
        vx1: &vx1,
        vvx1: &vvx1,
    };
    let Data_Out_Write_To_Output = to_string_pretty::<OutputData>(&Data_Write_To_Output)?;
    let mut Data_In_Write_To_Output = BufWriter::new(File::create("output.json")?);
    write!(&mut Data_In_Write_To_Output, "{}", Data_Out_Write_To_Output)?;
    println!("ok!");
    Ok(())
    
}



/////////////////////////////////////////////////////

#[derive(Serialize, Debug)]
struct WriteToInput <'a> {
    x1:&'a i32,
    vx1: &'a Vec<f64>,
    vvx1: &'a Vec<Vec<f64>>,
}

#[derive(Deserialize)]
struct InputData {
    x1: i32,
    vx1: Vec<f64>,
    vvx1: Vec<Vec<f64>>,
}

#[derive(Serialize, Debug)]
struct OutputData<'a> {
    x1:&'a i32,
    vx1: &'a Vec<f64>,
    vvx1: &'a Vec<Vec<f64>>,
}