use std::env;

mod fetch;
mod day01;

fn main() {
    eprintln!("\n*** Advent 2018 w/ Rust Bootstrap and Solver Tool ***");

    let default_part = String::from("1");

    let sess = env::var("ADV_SESS").expect("No ADV_SESS var found! Exiting...\n");
    let args: Vec<String> = env::args().collect();
    let day_input = args.get(1).expect("A day argument is required");
    let day_part_input = args.get(2).unwrap_or(&default_part);

    eprintln!("Solving Day {}, Part {}", day_input, day_part_input);

    let problem_input = fetch::input_for_day(&sess, day_input).expect("Error fetching input for problem!");

    println!("{}", day01::solve(&problem_input));
}
