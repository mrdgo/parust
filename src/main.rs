#![feature(rustc_private)]
mod thir_analysis;

fn main() {
    thir_analysis::thir_analysis();
    println!("Hello, world!");
}
