use std::io::Write;

pub fn read(message: &str) -> Vec<String> {
    println!("{} ", message);
    loop {
        // for ux (user experience)
        print!("> ");

        //clear the tests buffer so the print combined with the input above works
        std::io::stdout().flush().unwrap();

        //create a string for the input
        let mut input = String::new();

        //read the input
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        return parse_letters(input);
    }
}

pub fn parse_letters(letters: String) -> Vec<String> {
    //remove "[" & "]"
    let letters = letters.replacen("[", "", 1).replacen("]", "", 1);

    //find the separator
    let separator = if letters.contains(",") {
        ","
    } else {
        ""
    };

    // split the input into a Vec<&str> and remove the first and last element
    let mut letters: Vec<&str> = letters.trim().split(separator).collect::<Vec<&str>>();
    letters.remove(0);
    letters.remove(letters.len() - 1);

    // convert Vec<&str> to Vec<String>
    letters.iter().map(|&x| String::from(x)).collect::<Vec<String>>()
}