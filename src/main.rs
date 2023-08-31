use text_colorizer::*;
#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}
fn main() {
    print_help();
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".purple()
    );
    eprintln!("Usage: <target string> <replacement string> <input file> <output file>");
}
