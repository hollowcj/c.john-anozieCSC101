use std::io;

fn checker() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit",ch);

    }
    else {
        println!("Character '{}' is not a digit",ch);
    }
}

fn main() {
    // calling function
    println!("Welcome! This program checks whether a character variable contains a digit or not");
    checker()
}