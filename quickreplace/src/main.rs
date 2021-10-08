use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

fn main() {
    let args = match parse_args() {
        Some(a) => a,
        None => std::process::exit(1)
    };

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                      "ERROR".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "ERROR".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                      "ERROR".red().bold(), args.output, e);
            std::process::exit(1);
        }
    };
}

fn parse_args() -> Option<Arguments> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments received. Expected 4, got {}.", "ERROR:".red().bold(), args.len());
        return None
    }
    Some(Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    })
}

fn print_usage() {
    eprintln!("{} - change occurances of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>")
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}