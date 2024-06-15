use std::env;
use crate::scanner::parse_letters;

pub struct Args {
    pub url: String,
    pub letters: Vec<String>,
    pub print: bool,
}

impl Args {
    pub fn new() -> Args {
        let env = env::args().collect::<Vec<String>>();

        // create the variables
        let mut url = String::from("");
        let mut letters = Vec::<String>::new();
        let mut print = false;

        //get the arguments
        for i in env {
            if i.starts_with("--help") || i.starts_with("--h") {
                println!("LetterFinder is a tool to find words consisting of the given letters.");
                println!("Usage: LetterFinder [options]");
                println!("\nOptions:");
                println!("  --url=URL, -u=URL\t\tSet the URL to get the wordlist from.");
                println!("  --letters=LETTERS, -l=LETTERS\tSet the letters to find words consisting of.");
                println!("  --print, -p\t\t\tPrint the found words.");
                println!("  --help, -h\t\t\tShow this help message.");
                std::process::exit(0);
            }


            // if the argument starts with --url= or --u=, set the url to the value of the argument
            if i.starts_with("--url=") {
                url = i.replacen("--url=", "", 1);
            }

            if i.starts_with("--u=") {
                url = i.replacen("--u=", "", 1);
            }

            // if the argument starts with --letters or --l, set the letters to the value of the next argument
            if i.starts_with("--letters=") {
                letters = parse_letters(i.replacen("--letters=", "", 1));
            }

            if i.starts_with("--l=") {
                letters = parse_letters(i.replacen("--l=", "", 1));
            }

            if i.starts_with("--print") || i.starts_with("--p") {
                print = true;
            }
        }

        Args {
            url,
            letters,
            print,
        }
    }
}

