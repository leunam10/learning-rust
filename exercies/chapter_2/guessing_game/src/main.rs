/* 

*** Guessing Game *** 

The program will generate a random integer between 1 and 100. 
It will then prompt the player to enter a guess. After a guess is entered, 
the program will indicate whether the guess is too low or too high. 
If the guess is correct, the game will print a congratulatory message and exit.

*/

use std::io;


fn main() {
    println!("Welcome to the Guessing Game!!!");

    // Generate a random integer number between 1 and 100
    // For now we define the number to guess
    let to_guess = 10;

    // Variable to store the user attempt
    let mut guess = String::new();


    println!("Please input your guess: ");

    // reading user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //println!("Your guess is {guess}");

    // ********************************************************

    // this is by using the match statement + loop 

    // TO DO 

    // ********************************************************
    // this is by using the if statement + while loop 

    // converting the user input in a integer
    let guess_int: u32 = guess.trim().parse().expect("Please type a number!");

    // compare the guessed number with the number to guess
    if guess_int == to_guess{
        println!("YOU GUESSED THE RIGHT NUMBER: {guess}");
    }

    while guess_int != to_guess {

        // compare the guessed number with the number to guess
        if guess_int > to_guess{
            println!("to high");

        } else if guess_int < to_guess{
            println!("too low");
        
        } else {
            println!("YOU GUESSED THE RIGHT NUMBER: {guess} !!!");
        }

        println!("Please input your guess: ");

        // reading user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // converting the user input in a integer
        let guess_int: i32 = guess.trim().parse().expect("Please type a number!");
        
        println!("Guesss : {guess}");
        println!("Guess Integer: {guess_int}");
        println!("to guess: {to_guess}");
        println!("");
    }   


}
