use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[command(name = "sassenachfixer")]
#[command(about = "Fix yer English ↔ Scots", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Fix {
        text: String,
        #[arg(long)]
        to_scots: bool,
    },
}

fn load_dict() -> HashMap<String, String> {
    let data = fs::read_to_string("data/dictionary.json").expect("Couldnae read dictionary.json");
    serde_json::from_str(&data).expect("Bad JSON, mate")
}

fn reverse_dict(dict: &HashMap<String, String>) -> HashMap<String, String> {
    dict.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

fn translate(text: &str, dict: &HashMap<String, String>) -> String {
    text.split_whitespace()
        .map(|word| {
            let key = word.to_lowercase();
            dict.get(&key).cloned().unwrap_or(word.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let cli = Cli::parse();
    let dict = load_dict();

    match cli.command {
        Commands::Fix { text, to_scots } => {
            let active = if to_scots { reverse_dict(&dict) } else { dict };

            let result = translate(&text, &active);
            println!("{}", result);
        }
    }
}
