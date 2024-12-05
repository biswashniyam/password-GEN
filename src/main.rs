use rand::Rng;
use std::io;

fn main() {
    println!("===============================================");
    println!("|           Welcome to the Password           |");
    println!("|               Generator Tool!               |");
    println!("===============================================");

    println!("Enter the desired password length:\n");

    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Failed to read the input");

    let password_length: usize = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("🚫 Invalid input! Please enter a valid number.");
            return;
        }
    };

    // Combination of characters to form the password
    let lowercase_char = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()_-+=<>?/{}~";

    // Combine all character sets for generating a strong password
    let combi_chars = format!("{}{}{}{}", lowercase_char, uppercase_char, numbers, symbols);

    let password = generate_password(password_length, &combi_chars);

    // Print the generated password
    println!("------------------------------------------------");
    println!("Your generated password is:\n");
    println!("==> {}\n",password);
    println!("------------------------------------------------");
    println!("Use this strong password to stay secure! 🔒");
    println!("------------------------------------------------");
}

// Function to generate a random password
fn generate_password(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset.chars().nth(idx).unwrap());
    }
    password // return is optional; just `password` is also fine
}
