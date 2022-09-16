#![allow(non_snake_case)]
// #[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
// use std::io::BufReader;
//
use serde::Serialize;
use serde_json::to_string_pretty;
// use serde::Deserialize;
// use serde_json::from_reader;


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

    // println!("{:?}",Data_Write_To_Input);

    Ok(())
    
}

/////////////////////////////////////////////////////

#[derive(Serialize, Debug)]
struct WriteToInput <'a> {
    x1:&'a i32,
    vx1: &'a Vec<f64>,
    vvx1: &'a Vec<Vec<f64>>,
}
