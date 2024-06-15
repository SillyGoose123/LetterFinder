use std::panic;
use std::process::exit;
use crate::args::Args;
use crate::finder::find_words;

mod scanner;
mod args;
mod finder;

fn main() {
    //custom error handling
    panic::set_hook(Box::new(|info| {
        match &info.to_string().split("\n").collect::<Vec<&str>>().last() {
            Some(error) => eprintln!("{}", error),
            None => eprintln!("Unknown error"),
        }

        // kill the program with exit code 1
        exit(1);
    }));

    let args: Args = Args::new();

    if !&args.letters.is_empty() {
        let words = find_words(&args, &args.letters);
        print_words(&args.print, words);
        return;
    }

    println!("Hello from LetterFinder!");
    loop {
        //get the letters from the user
        let input: Vec<String> = scanner::read("Enter letters to find words consisting of those letters: ");

        //show found words
        let words = find_words(&args, &input);
        print_words(&args.print, words);

        println!("\n-------------------------------------------------------\n")
    }
}

fn print_words(print: &bool, words: Vec<String>) {
    if *print {
        for word in words {
            println!("{}", word);
        }
    } else {
        println!("Found {} words", words.len());
    }
}