mod config;
mod orchestrator;

use config::Config;
use serde_yaml;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use uuid::Uuid;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <config_file> <mode>", args[0]);
        std::process::exit(1);
    }

    let config_file = &args[1];
    let mode = &args[2];

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
        Ok(ref config) => {
            println!("Config loaded: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            std::process::exit(1);
        }
    }

    if mode == "train" {
        // Train the model
    } else if mode == "talk" {
        let mut orchestrator = orchestrator::Orchestrator::new(uuid::Uuid::new_v4(), config);
        while true {
            orchestrator.run().await;
        }
    } else if mode == "db" {
        // Query the database
    } else {
        eprintln!("Invalid mode");
        std::process::exit(1);
    }
}
