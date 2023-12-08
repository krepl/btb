use btb::tokenizer::Tokenizer;
use std::io;

fn main() {
    println!("Please enter some text:");

    // Create a mutable String to store the input
    let mut input = String::new();

    // Read a line into the string
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // The input operation was successful, do something with the input
            let mut tokenizer = Tokenizer::new(input.as_ref());

            while let Some(token) = tokenizer.next_token() {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            // The input operation failed, handle the error
            println!("Error: {}", e);
        }
    }
}
