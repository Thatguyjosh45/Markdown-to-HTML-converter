use std::env;
use std::fs;
use pulldown_cmark::{Parser, Options, html};

fn main() {
    // Read args: markdown_converter input.md output.html
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.md> <output.html>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Read Markdown file
    let markdown_input = fs::read_to_string(input_path)
        .expect("Failed to read input file");

    // Set up options and parser
    let options = Options::all();
    let parser = Parser::new_ext(&markdown_input, options);

    // Render HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Write HTML to output file
    fs::write(output_path, html_output).expect("Failed to write output file");

    println!("Converted {} to {}", input_path, output_path);
}
