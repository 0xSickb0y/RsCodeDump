use std::io;
use std::process;

const MAX_ERRORS: u8 = 7;
const SECRET_WORD: &str = "\x66\x6c\x6f\x70\x70\x61";

fn banner() {
    print!("Welcome to hangman_game.rs, Can you guess what the secret word is?\n");
    print!("Max errors: 7\n\n");
    print!("⠈⢿⣿⣿⣿⣿⣿⣿⠄⠀⠀⠀⠀⠀⢀⣀⣀⣀⠀⣀⣀⣴⣼⣿⣿⣿⣿⣿⣿⡏\n");
    print!("⠀⠘⣷⢿⣿⣿⣿⣿⣤⣤⣶⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗\n");
    print!(" ⠀⢸⠟⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇\n");
    print!(" ⠀⠈⡷⢲⣿⣿⣯⣿⣿⣿⣿⠛⢻⣿⣿⣿⣿⣿⣿⣿⣿⠉⢻⣿⣿⣿⣿⣿⡁\n");
    print!("  ⠀⠈⢋⣾⡿⢿⣿⣿⣿⡇⠈⣿⣿⣿⣿⣿⣿⣿⣿⡀⣾⣿⣿⣿⣿⣿⣿:\n");
    print!("  ⠀⠀⠀⣠⣾⣿⠧⠟⠛⠉⢙⠁⢶⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠂⡉⣿⣿⣿⣿⡄\n");
    print!("  ⠀⠀⠀⣿⢻⣧⣶⣄⢀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⠇⢤⣀⣴⣽⣿⣿⣿⣿⣿⣆\n");
    print!("  ⠀⠀⢠⣿⠸⣿⣿⣿⣷⣦⣄⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⠙⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀\n");
    print!("  ⠀⠀⣾⣿⡀⠹⠿⢿⣿⣿⣿⡆⠀⠘⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦\n");
    print!("  ⠀⢰⣿⣿⣿⡀⠀⠘⠻⣿⡿⠃⠀⠀⠙⢿⣿⣿⣿⣿⠇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("  ⠀⣼⣿⣿⣿⣿⣦⣄⠀⠀⠀⢲⣤⡀⠀⠀⢿⣿⣿⡿⢂⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("  ⠀⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠛⢿⣷⣄⠀⠀⠈⣁⣤⣾⣿⣿⣿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠰⠿⣿⣿⣷⣄⠀⣸⣿⣿⣿⣿⡟⣹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡀⠐⠛⢛⢋⡿⠃⢿⣭⣿⡿⠉⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("  ⠀⣿⣿⣯⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣼⣿⣵⣶⣶⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");
    print!("");
}

fn input_guess() -> char {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let trimmed = input.trim().to_lowercase();

    if trimmed.len() == 1 {
        let character = trimmed.chars().next().unwrap();
        return character;

    } else {
        println!("Please enter a valid single character.");
        return input_guess()
    }
}


fn print_hangman(secret_word: &str, alphabet_vec: &Vec<char>) {
    for i in secret_word.chars() {
        if alphabet_vec.contains(&i) {
            print!("{} ", i);
        } else {
            print!("_ ");
        }
    }
    println!("\n");
}

fn process_guess(character: char, secret_word: &str, alphabet_vec: &mut Vec<char>, tries: &mut u8) {
    if !alphabet_vec.contains(&character) {
        alphabet_vec.push(character);

        if !secret_word.contains(character) {
            *tries += 1;
            println!("Incorrect guess! Tries left {}", MAX_ERRORS - *tries);
        }
    }
}

fn is_hanged(tries: u8, alphabet_vec: &Vec<char>) {
    if tries >= MAX_ERRORS {
        println!("\nYou hanged, try again!");
        process::exit(0);
    }

    if SECRET_WORD.chars().all(|c| alphabet_vec.contains(&c)) {
        println!("\nYou won! The Secret word was FLOPPA");
        process::exit(0);
    }
}
fn main() {
    banner();
    let mut alphabet_vec: Vec<char> = Vec::new();
    let mut tries: u8 = 0;

    loop {
        print_hangman(SECRET_WORD, &alphabet_vec);
        let character = input_guess();
        process_guess(character, SECRET_WORD, &mut alphabet_vec, &mut tries);
        is_hanged(tries, &alphabet_vec);

    }
}
