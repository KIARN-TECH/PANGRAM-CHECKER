
use std::collections::HashSet;

fn _miracle (arnold: &str) -> bool {
    // Create a set to store all unique letters from the sentence
    let mut letters = HashSet::new();

    // Iterate over the characters in the sentence
    for x in arnold.chars() {
        // Convert the character to lowercase and check if it's a letter
        if x.is_ascii_alphabetic() {
            letters.insert(x.to_ascii_lowercase());
        }
    }

    // Check if the set contains all 26 letters of the alphabet
    letters.len() == 26
}

fn main() {
    // Example sentences to test
    let sentence1 = "The quick brown fox jumps over the lazy dog";
    let sentence2 = "Hello world";

    println!("Is pangram (sentence1): {}", _miracle ( sentence1)); // true
    println!("Is pangram (sentence2): {}", _miracle (sentence2)); // false
    }

