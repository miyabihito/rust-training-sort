#[macro_use]
extern crate clap;
extern crate rust_training_sort;

use clap::Arg;
use rust_training_sort::Input;

fn main() {
    let m = app_from_crate!()
        .arg(Arg::with_name("target")
                .required(true)
                .help("sort target numbers")
        )
        .get_matches();

    let target = m.value_of("target").unwrap();
    // TODO: validation
    let target = target.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
    let input = Input { target };

    rust_training_sort::run(input);
}
