use std::panic;
use regex::{RegexBuilder};
use crate::args::Args;

pub fn find_words(args: &Args, letters: &Vec<String>) -> Vec<String> {
    let mut words = Vec::<String>::new();
    let all_words = get_all_words(&args.url);

    let re = RegexBuilder::new(format!(r"\b[{}+]+\b", &letters.join("")).as_str())
        .case_insensitive(true)
        .build()
        .unwrap();

    //match the words and add them to words
    re.find_iter(&all_words).for_each(|word| {
        words.push(word.as_str().to_string());
    });

    words
}

fn get_all_words(url: &String) -> String {
    let url: String = if url.is_empty() {
        String::from("https://raw.githubusercontent.com/jeremy-rifkin/Wordlist/master/master.txt")
    } else {
        url.clone()
    };

    let text: String = match reqwest::blocking::get(url) {
        Ok(response) => response.text().unwrap(),
        Err(_) => panic!("Failed to get the wordlist."),
    };

    text
}