use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines().map(|l| l.unwrap());

    let priorities = build_priorities();

    let score = lines.fold(0, |acc, l| {
        let (compartment_a, compartment_b) = l.split_at(l.len() / 2);

        let found_chars = str_to_char_map(compartment_a);

        let duplicate = compartment_b
            .chars()
            .find(|c| found_chars.get(c).is_some())
            .expect("All rucksacks are expected to contain a duplicate item.");

        acc + *priorities.get(&duplicate).unwrap()
    });

    println!("The total of the duplicate item scores is: {score}.");

    Ok(())
}

// Builds a map of chars to their point values, for the point values given in the challenge brief.
fn build_priorities() -> HashMap<char, usize> {
    let mut priorities = HashMap::new();
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for (i, char) in alpha.chars().enumerate() {
        priorities.insert(char, i + 1);
    }

    priorities
}

// Creates a map of char to `true` for every char in a given &str.
fn str_to_char_map(s: &str) -> HashMap<char, bool> {
    let mut m = HashMap::new();

    for c in s.chars() {
        m.insert(c, true);
    }
    m
}
