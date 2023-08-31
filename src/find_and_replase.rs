use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;
#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}
fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".purple()
    );
    eprintln!("Usage: <target string> <replacement string> <input file> <output file>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect(); //INFO we skip for the run comment we do nto there want
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{}: Expected 4 arguments, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}
fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}: Failed to read from file {}: {}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(1);
        }
    };
    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "{}: Failed to replace {} with {} in {}: {}",
                "Error".red().bold(),
                args.pattern,
                args.replace,
                args.input_file,
                e
            );
            std::process::exit(1);
        }
    };
    match fs::write(&args.output_file, replace_data) {
        Ok(_) => eprint!(
            "{}: Successfully wrote to file {}",
            "Success".yellow().bold(),
            args.output_file
        ),
        Err(e) => {
            eprintln!(
                "{}: Failed to write to file {}: {}",
                "Error".red().bold(),
                args.output_file,
                e
            );
            std::process::exit(1);
        }
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}

pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}
