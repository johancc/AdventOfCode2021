use std::env;

mod common;
mod solutions;

fn main() {
    println!("Advent of Code 2021 Solutions");
    println!("      Johan Cervantes      \n");

    if let Some(run_arg) = env::args().nth(1) {
        let part = run_arg.chars().last().unwrap().to_string();
        let mut run_arg = run_arg.chars();
        run_arg.next_back().unwrap();
        return run(run_arg.as_str().parse().unwrap(), part);
    }

    for (i, item) in solutions::ALL.iter().enumerate() {
        println!("[{}] {}", i, item.name());
    }

    let run_index = common::input("\nIndex ").unwrap();
    let run_index = match run_index.parse::<usize>() {
        Ok(i) => {
            if i >= solutions::ALL.len() {
                return println!("Invalid index.");
            }
            i
        }
        Err(_) => return println!("Unable to parse index."),
    };
    let part = common::input("Part (A / B) > ").unwrap();
    run(run_index, part);
}

fn run(run_index: usize, part: String) {
    let solution = solutions::ALL[run_index];
    println!("Running {} ({})", solution.name(), part.to_uppercase());

    let start = std::time::Instant::now();
    let out = match part.to_lowercase().as_str() {
        "a" => solution.part_a(),
        "b" => solution.part_b(),
        _ => return println!("Invalid Part"),
    };
    let delta = start.elapsed().as_nanos();
    println!("Out: {} ({})", out, common::time_unit(delta));
}
