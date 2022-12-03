use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let priorities = build_priorities();

    let score_1 = part_1(&priorities);
    let score_2 = part_2(&priorities);

    println!("The total of the duplicate item scores is: {score_1}.");
    println!("The total of the ID badge scores is: {score_2}.");

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

fn part_1(priorities: &HashMap<char, usize>) -> usize {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines.map(|l| l.unwrap()).fold(0, |acc, l| {
        let (compartment_a, compartment_b) = l.split_at(l.len() / 2);

        let found_chars = str_to_char_map(compartment_a);

        let duplicate = compartment_b
            .chars()
            .find(|c| found_chars.get(c).is_some())
            .expect("All rucksacks are expected to contain a duplicate item.");

        acc + *priorities.get(&duplicate).unwrap()
    })
}

fn part_2(priorities: &HashMap<char, usize>) -> usize {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines
        .map(|l| l.unwrap())
        .map(|rucksack| str_to_char_map(rucksack.as_str()))
        .collect::<Vec<HashMap<char, bool>>>()
        .chunks(3)
        .fold(0, |acc, chunk| {
            let mut counts = HashMap::new();
            for sack in chunk {
                for item in sack.keys() {
                    let count = counts
                        .entry(item)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    if *count == 3 {
                        return acc + *priorities.get(item).unwrap();
                    }
                }
            }

            acc
        })
}
