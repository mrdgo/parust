#![feature(rustc_private)]

use std::{env, fs};
mod thir_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &String::from(&args[1]);
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    // TODO: think about ideal distribution of file_name and contents
    thir_analysis::thir_analysis(file_name, contents);
}
