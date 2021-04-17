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
        .arg(Arg::with_name("order")
                .short("o")
                .takes_value(true)
                .possible_values(&["asc", "desc"])
                .help("specify order: asc or desc")
        )
        .get_matches();

    let target = m.value_of("target").unwrap();
    // TODO: validation
    let target = target.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();

    let asc = match m.value_of("order") {
        Some("desc") => false,
        _ => true,
    };

    let input = Input { target, asc };

    rust_training_sort::run(input);
}
