use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines().map(|l| l.unwrap());

    let mut high_elves = [0; 3];
    let mut curr_elf_cals = 0;

    for line in lines {
        if line.trim().is_empty() {
            for ranked_elf_cals in &mut high_elves {
                if curr_elf_cals > *ranked_elf_cals {
                    (*ranked_elf_cals, curr_elf_cals) = (curr_elf_cals, *ranked_elf_cals);
                }
            }
            curr_elf_cals = 0;
        } else if let Ok(cals) = line.parse::<i32>() {
            curr_elf_cals += cals;
        } else {
            panic!("Line contains invalid input.");
        }
    }
    let highest: i32 = high_elves.into_iter().sum();

    println!("The sum of the highest calorie totals is: {highest}.");

    Ok(())
}
