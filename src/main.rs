use rand::Rng; // For generating random numbers
use std::io;   // For reading user input

// This function generates a secret code of a given length made of random digits (0-9)
fn generate_code(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng(); // Random number generator
    let mut code = Vec::new(); // Start with an empty list (vector) of digits

    for _ in 0..length {
        let digit = rng.gen_range(0..10); // Pick a random number from 0 to 9
        code.push(digit); // Add it to the code
    }

    code // Return the code
}

// This function checks how many digits are correct and in the correct position (green),
// and how many digits are correct but in the wrong position (yellow)
fn evaluate_guess(secret: &[u8], guess: &[u8]) -> (usize, usize) {
    let mut green = 0; // Correct digit and position
    let mut yellow = 0; // Correct digit but wrong position

    let mut secret_used = vec![false; secret.len()];
    let mut guess_used = vec![false; guess.len()];

    // First pass: check for greens
    for (i, (&s, &g)) in secret.iter().zip(guess.iter()).enumerate() {
        if s == g {
            green += 1;
            secret_used[i] = true;
            guess_used[i] = true;
        }
    }

    // Second pass: check for yellows
    for (i, &g) in guess.iter().enumerate() {
        if guess_used[i] {
            continue; // Skip if already counted as green
        }

        for (j, &s) in secret.iter().enumerate() {
            if !secret_used[j] && g == s {
                yellow += 1;
                secret_used[j] = true;
                break;
            }
        }
    }

    (green, yellow)
}

// This function reads a valid guess from the user
fn read_guess(length: usize) -> Vec<u8> {
    loop {
        println!("Enter your guess ({} digits):", length);

        let mut input = String::new(); // A place to store what the user types
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // Remove any spaces or newlines

        // Make sure the input is the right length and all digits
        if input.len() != length || !input.chars().all(|c| c.is_digit(10)) {
            println!("Invalid input. Try again.");
            continue; // Go back to the top of the loop
        }

        // Convert each character into a digit
        let guess: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        return guess; // Return the guess
    }
}

// The main function is where the program starts running
fn main() {
    println!("🎯 Welcome to Mastermind!");
    println!("Choose a code length:");
    println!("1. 3 digits");
    println!("2. 5 digits");
    println!("3. 7 digits");

    let length = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim() {
            "1" => break 3,
            "2" => break 5,
            "3" => break 7,
            _ => {
                println!("Invalid choice. Enter 1, 2, or 3.");
                continue;
            }
        }
    };

    let secret = generate_code(length); // Create the secret code
    let max_attempts = 10;

    println!("\nGreat! I've picked a {}-digit secret code.", length);
    println!("You have {} attempts to guess it.", max_attempts);
    println!("🟩 = Correct digit in correct place");
    println!("🟨 = Correct digit but wrong place\n");

    for attempt in 1..=max_attempts {
        println!("🔢 Attempt {}/{}:", attempt, max_attempts);
        let guess = read_guess(length); // Get the user's guess

        let (green, yellow) = evaluate_guess(&secret, &guess); // Check the guess

        if green == length {
            println!("🎉 You guessed it! The code was {:?}.", secret);
            return; // End the program
        } else {
            println!("🟩 {} correct and in the right position", green);
            println!("🟨 {} correct but in the wrong position\n", yellow);
        }
    }

    println!("❌ You've used all attempts! The secret code was {:?}.", secret);
}

