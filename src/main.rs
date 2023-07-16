use serde_json::Value;
use reqwest::blocking::Client;
use colored::Colorize;
use std::env;

fn define(word: &str, lang: &str) {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/{lang}/{word}");

    let client = Client::new();

    let res: Value = client
        .get(url)
        .send()
        .expect("Unable to send request")
        .json()
        .unwrap();

    let meanings = &res[0]["meanings"];

    if let Value::Null = meanings {
        // Invalid word or langauge
        println!("No definition found for \"{}\"", word.red().italic().bold());
        return;
    }

    for meaning in meanings.as_array().unwrap() {
        let part_of_speech = meaning["partOfSpeech"].as_str().unwrap();
        println!("{}", part_of_speech.bold().italic().underline().magenta());

        for definition in meaning["definitions"].as_array().unwrap() {
            println!("  -  {}", definition["definition"].as_str().unwrap());

            if definition.as_object().unwrap().contains_key("example") {
                println!("     example: \"{}\"", definition["example"].as_str().unwrap().italic().dimmed());
            }

        }

    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid usage.");
        println!("usage: {} <word>", args[0]);
        return;
    }

    let word = &args[1];

    let lang = "en";

    define(word, lang);
}
