extern crate rust_training_sort;

use std::env;

fn main() {
    let mut input_iter = env::args();
    input_iter.next();
    let mut input = input_iter.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    rust_training_sort::run(&mut input);

    println!("{:?}", input);
}
