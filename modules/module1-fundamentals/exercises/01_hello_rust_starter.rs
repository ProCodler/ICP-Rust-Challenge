use std::io;

fn main() {
    // 1. Prompt the user for their name
    println!("What is your name?");

    // 2. Read the user's input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    // 3. Print a personalized greeting
    println!("Hello, {}! Welcome to Rust!", name);

    // BONUS: Print the current date
    // Hint: You can use the chrono crate for this
    match chrono::Local::now().date_naive().to_string().parse::<String>() {
        Ok(date) => println!("Today's date is: {}", date),
        Err(_) => println!("Could not get the current date."),
    }
}