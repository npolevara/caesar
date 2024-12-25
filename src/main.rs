use clap::{Command, Arg};
use regex::Regex;

fn encrypt(text: &str, shift: i32) -> String {
    let shift = (shift % 26 + 26) % 26;
    let regex = Regex::new(r"[a-zA-Z]").unwrap();

    let lower: Vec<char> = ('a'..='z').collect();
    let upper: Vec<char> = ('A'..='Z').collect();

    text.chars()
        .map(|c| {
            if regex.is_match(&c.to_string()) {
                let letters = if c.is_uppercase() { &upper } else { &lower };
                let pos = letters.iter().position(|&x| x == c).unwrap();
                letters[(pos + shift as usize) % 26]
            } else {
                c
            }
        })
        .collect()
}

fn print_result(description: &str, result: &str){
    println!("{description} {result}");
}

fn main() {
    let matches = Command::new("Caesar chiper")
        .version("0.0.1")
        .arg(
            Arg::new("prompt")
                .short('p')
                .long("prompt")
                .value_name("PROMPT")
                .help("The text prompt to encrypt/decrypt")
                .required(true),
        )
        .arg(
            Arg::new("shift")
                .short('s')
                .long("shift")
                .value_name("SHIFT")
                .help("The number of positions to shift characters 0 - 25")
                .required(true),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .help("Decrypt the text instead of encrypting it")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let decrypt: bool = matches.get_flag("decrypt"); 
    let prompt: String = matches.get_one::<String>("prompt").unwrap().to_string();
    let description: String;
    let result: String;
    let shift: i32 = matches.get_one::<String>("shift").unwrap().parse::<i32>().unwrap();
    
    if decrypt {
        description = format!("Decrypting from \"{prompt}\" to: ");
        result = encrypt(&prompt, -shift);
        print_result(&description, &result);

        return
    }

    description = format!("Encrypting from \"{prompt}\" to: ");
    result = encrypt(&prompt, shift);
    print_result(&description, &result);
}
