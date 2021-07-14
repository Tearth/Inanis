use crate::{movegen, perft};
use chrono::Utc;
use std::{io, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DATE: &str = env!("DATE");
const HASH: &str = env!("HASH");

pub fn init() {
    println!("Ina v{} ({}), created by {}", VERSION, DATE, AUTHOR);
    println!("Executable hash: {}", HASH);
    println!("Homepage: https://github.com/Tearth/Ina");
    println!();
    println!("Type \"help\" to get a list of available commands");
}

pub fn run() {
    loop {
        let input = read_line();
        let split: Vec<&str> = input.split(' ').collect();
        let trimmed = split[0].trim().to_lowercase();

        match trimmed.as_str() {
            "help" => handle_help(),
            "magic" => handle_magic(),
            "perft" => handle_perft(split),
            "wah" => handle_wah(),
            "quit" => handle_quit(),
            _ => handle_unknown_command(),
        }
    }
}

fn handle_help() {
    println!("List of available commands:");
    println!("  magic - generate magic numbers");
    println!("  perft d - run perft test with d depth");
    println!("  quit - close the application");
}

fn handle_magic() {
    let now = Utc::now();
    println!("Generating magic numbers for rook...");

    for index in 0..64 {
        println!("{},", movegen::generate_rook_number_for_field(index));
    }

    println!();
    println!("Generating magic numbers for bishop...");

    for index in 0..64 {
        println!("{},", movegen::generate_bishop_number_for_field(index));
    }

    let diff = (Utc::now() - now).num_milliseconds();
    println!("Done! Magic numbers generated in {} ms", diff);
}

fn handle_perft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    let max_depth: i32 = match input[1].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid depth parameter");
            return;
        }
    };

    for depth in 1..max_depth + 1 {
        let now = Utc::now();
        let count = perft::run(depth);
        let diff = ((Utc::now() - now).num_milliseconds() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!("Depth {}: {} leafs in {:.2} s ({:.2} ML/s)", depth, count, diff, mnps);
    }

    println!("Perft done!");
}

fn handle_wah() {
    println!("WAH");
}

fn handle_quit() {
    process::exit(0);
}

fn handle_unknown_command() {
    println!("Unknown command, type \"help\" to get a list of available ones");
}

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer
}
