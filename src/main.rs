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
    let mut result = Vec::new();

    for token in text.split_whitespace() {
        let mut word = token.to_string();

        let mut prefix = String::new();
        let mut suffix = String::new();

        // handle prefix punctuation
        if let Some(first) = word.chars().next() {
            if !first.is_alphanumeric() {
                prefix.push(first);
                word = word.chars().skip(1).collect();
            }
        }

        // handle suffix punctuation
        if let Some(last) = word.chars().last() {
            if !last.is_alphanumeric() {
                suffix.insert(0, last);
                word = word.chars().take(word.len() - 1).collect();
            }
        }

        let lower = word.to_lowercase();

        let mut translated = dict
            .get(&lower)
            .cloned()
            .unwrap_or(word.clone());

        // capitalisation fix (safe)
        if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            let mut chars: Vec<char> = translated.chars().collect();
            if let Some(first) = chars.get_mut(0) {
                first.make_ascii_uppercase();
            }
            translated = chars.into_iter().collect();
        }

        let final_word = format!("{}{}{}", prefix, translated, suffix);
        result.push(final_word);
    }

    result.join(" ")

}fn main() {
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
