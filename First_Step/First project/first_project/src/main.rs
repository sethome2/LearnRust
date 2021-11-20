use std::io;

fn main() {
    println!("Hello, world!");
    let mut a = String::new();
    
    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    println!("You guessed: {}", a);
}
