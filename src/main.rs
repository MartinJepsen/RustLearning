use std::{io, cmp::Ordering}; // imports io into the namespace
use rand::Rng;

fn main() {
    // main() is the entry point to the programme
    println!("Guess the number!");

    // thread_rng is an rng that is local to the thread and takes a seed from the OS
    // The range is start..=end and is inclusive.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    
    // let mut creates a mutable variable
    // String::new() creates a new Sting instance, which is a growable piece of text.
    // The ::new syntax means that new() is an associated function of the String type.
    // An associated function is a function that is implemented on a type.
    let mut guess = String::new();

    // read_line(&mut guess) means that we store the user input in the guess variable
    // The & indicates a reference. This lets multiple parts of the programme access the same data without having to copy it.
    // The .expect function tells the programme what to do if we get an exception.
    // This is because read_line returns Result which is an enum. This enum has either the user input or an error
    // and .expect handles this error.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // The Ordering type is an enum and hass variants Less, Greater and Equals.
    // The cmp method compares two values. It can be called on anything that can be compared.
    // It returns a variant of Ordering.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct!"),
    }
}