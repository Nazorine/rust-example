#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
//
use serde::Deserialize;  // Input
use serde::Serialize;    // Output
use serde_json::from_reader;
use serde_json::to_string_pretty;


fn main() -> Result<(), Box<dyn Error>>  {

    /////从input.json文件读取输入数据
    let Data_Read_From_Input = BufReader::new(File::open("input.json")?);
    let InputData {
        x1,
        vx1,
        vvx1,
    } = from_reader::<_, InputData>(Data_Read_From_Input)?;



    /////将结果写入ouput.json
    let Data_Write_To_Output = OutputData {
        x1:&x1,
        vx1: &vx1,
        vvx1: &vvx1,
    };
    let Data_Write_To_Output_Out = to_string_pretty::<OutputData>(&Data_Write_To_Output)?;
    let mut Data_Write_To_Output_In = BufWriter::new(File::create("output.json")?);
    write!(&mut Data_Write_To_Output_In, "{}", Data_Write_To_Output_Out)?;
    println!("ok!");
    Ok(())
    
}


/////////////////////////////////////////////////////

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