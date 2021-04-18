#[macro_use]
extern crate clap;
extern crate rust_training_sort;

use clap::Arg;
use rust_training_sort::Input;

fn main() {
    let m = app_from_crate!()
        .arg(Arg::with_name("target")
                .required(true)
                .multiple(true)
                .validator(|x| {
                    match x.parse::<i32>() {
                        Ok(_) => Ok(()),
                        Err(_) => Err(String::from("need numbers")),
                    }
                })
                .help("sort target numbers")
        )
        .arg(Arg::with_name("order")
                .short("o")
                .long("order")
                .takes_value(true)
                .possible_values(&["asc", "desc"])
                .help("specify order")
        )
        .arg(Arg::with_name("algorithm")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .default_value("default")
                .possible_values(&["default", "bubble"])
                .help("sort algorithm")
        )
        .get_matches();

    let target = m.values_of("target")
                    .unwrap()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

    let asc = match m.value_of("order") {
        Some("desc") => false,
        _ => true,
    };

    let algorithm = m.value_of("algorithm").unwrap().to_string();

    let input = Input { target, asc, algorithm };

    let output = rust_training_sort::run(input);
    let output = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
    println!("{}", output);
}
