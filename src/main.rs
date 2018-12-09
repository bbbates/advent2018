use std::env;

mod day01;

fn main() {
    eprintln!("\n*** Advent 2018 w/ Rust Bootstrap and Solver Tool ***");

    let default_part = String::from("1");

    let session = env::var("ADV_SESS").expect("No ADV_SESS var found! Exiting...\n");
    let args: Vec<String> = env::args().collect();
    let day_input = args.get(1).expect("A day argument is required");
    let day_part_input = args.get(2).unwrap_or(&default_part);

    eprintln!("Solving Day {}, Part {}", day_input, day_part_input);

    println!("{}", day01::solve(&session.as_str()))
}
