use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Secret: {secret_num}");
    let mut guess: String = String::from("0");//String::new();
    //&mut guess = "0";

    let mut guesses: i32 = 0;

    while guess.trim().parse::<i32>().expect("please give me correct string number!") != secret_num  {

        if !(guesses > 0) {
            println!("Please input a guess between 1 - 100.");
        }
        else if guess.trim().parse::<i32>().expect("please give me correct string number!") > secret_num {
            println!("Wrong! Try Lower...");
        }
        else if guess.trim().parse::<i32>().expect("please give me correct string number!") < secret_num {
            println!("Wrong! Try Higher...");
        }
        guess.clear();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        guess = String::from(guess.trim());

        guesses += 1;
    }

    println!("You guessed the correct number, {guess}, in {guesses} trys.")

}