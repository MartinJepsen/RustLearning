use std::io; // imports io into the namespace

fn main() {
    // main() is the entry point to the programme
    println!("Guess the number!");
    println!("Please input your guess.");
    
    // let mut creates a mutable variable
    // String::new() creates a new Sting instance, which is a growable piece of text.
    // The ::new syntax means that new() is an associated function of the String type.
    // An associated function is a function that is implemented on a type.
    let mut guess = String::new();

    // read_line(&mut guess) means that we store the user input in the guess variable
    // The & indicates a reference. This lets multiple parts of the programme access the same data without having to copy it.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}