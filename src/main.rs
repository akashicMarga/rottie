mod config;
mod db;
mod data;
mod utils;
mod inference;
mod train;


use std::env;
use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
use config::Config;

use rottie_inference::phi3;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        std::process::exit(1);
    }

    let config_file = &args[1];
    let mut file = File::open(config_file).unwrap_or_else(|_| {
        eprintln!("Unable to open the file");
        std::process::exit(1);
    });
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Unable to read the file: {}", e);
        std::process::exit(1);
    }

    let config: Result<Config, _> = serde_yaml::from_str(&contents);
    match config {
        Ok(config) => {
            println!("Config loaded: {:?}", config);
        },
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            std::process::exit(1);
        }
    }

    // Create an instance of the phi3 struct
    let mut phi3 = rottie_inference::phi3::phi3::init();

    use std::io::{self};

    let mut input_text = String::new();
    println!("Enter text to generate with Phi3 model or type 'quit' to exit:");

    while input_text.to_lowercase().trim() != "quit" {
        print!("Enter prompt: ");
        io::stdout().flush().unwrap();
        input_text.clear();
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        if input_text.to_lowercase().trim() == "quit" {
            break;
        }

        match phi3.generate(input_text.clone()) {
            Ok(response) => println!("Generated text: {:?}", response),
            Err(e) => eprintln!("Error generating text: {}", e),
        }
    }

    
}