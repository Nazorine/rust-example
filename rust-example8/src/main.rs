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


    let x1_ = 1;
    let vx1_ = vec![1.0,2.0,3.0];
    let vvx1_ = vec![vec![1.0,2.0,3.0]];

    let Data_Write_To_Input = WriteToInput {
        x1:&x1_,
        vx1: &vx1_,
        vvx1: &vvx1_,
    };
    let Data_Write_To_Input_Out = to_string_pretty::<WriteToInput>(&Data_Write_To_Input)?;
    let mut Data_Write_To_Input_In = BufWriter::new(File::create("input.json")?);
    write!(&mut Data_Write_To_Input_In, "{}", Data_Write_To_Input_Out)?;



    /////从input.json文件读取输入数据
    let Data_Read_From_Input = BufReader::new(File::open("input.json")?);
    let InputData {
        x1,
        vx1,
        vvx1,
    } = from_reader::<_, InputData>(Data_Read_From_Input)?;

    println!("{:?},{:?},{:?}", x1,vx1,vvx1); 



    // /////将结果写入ouput.json
    // let Data_Write_To_Output = OutputData {
    //     x1:&x1,
    //     vx1: &vx1,
    //     vvx1: &vvx1,
    // };
    // let Data_Write_To_Output_Out = to_string_pretty::<OutputData>(&Data_Write_To_Output)?;
    // let mut Data_Write_To_Output_In = BufWriter::new(File::create("output.json")?);
    // write!(&mut Data_Write_To_Output_In, "{}", Data_Write_To_Output_Out)?;
    // println!("ok!");
    Ok(())
    
}


/////////////////////////////////////////////////////

#[derive(Serialize, Debug)]
struct WriteToInput<'a> {
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