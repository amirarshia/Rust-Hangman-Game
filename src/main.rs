use std::io;

fn main() {
    let mut guess = String::new();
    let mut fail_number = 0;
    let mut word = String::new();
    let mut win_number = 0;
    let mut attempts = 0;
    
    // Start of program
    println!("Hello, welcome to HANGMAN!");
    // Getting the word thats going to be guessed
    println!("Insert word that you want the other player to be guessing: ");
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word_len = word.len();
    loop{
        let char_wordletter = word.chars().nth(win_number).unwrap();
        attempts = attempts + 1;
        println!("Insert guess: ");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let char_guess = guess.chars().nth(0).unwrap();
        if char_guess == char_wordletter {
            win_number = win_number + 1;
            println!("Nice job, you got the {} letter correct!", win_number);

        } else {
            fail_number = fail_number + 1;
            println!("WRONG! {} trys remaining!", 6 - fail_number);
        } 
        let winning = word_len - 1;
        if winning == win_number {
            println!("You have won with {} trys!", attempts);
            break;
        }
        guess = String::new();

    }
}
