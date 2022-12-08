use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines().map(|l| l.unwrap());

    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}),(\d{1,2})-(\d{1,2})$").unwrap();

    let mut contain_count = 0;
    let mut overlap_count = 0;

    for line in lines {
        let captures = re
            .captures(&line)
            .expect("Every line should contain 2 valid ranges.");

        println!("{line}");

        let range_a_start: isize = captures.get(1).unwrap().as_str().parse().unwrap();
        let range_a_end: isize = captures.get(2).unwrap().as_str().parse().unwrap();
        let range_b_start: isize = captures.get(3).unwrap().as_str().parse().unwrap();
        let range_b_end: isize = captures.get(4).unwrap().as_str().parse().unwrap();

        let a_contains_b = range_a_start >= range_b_start && range_a_end <= range_b_end;
        let b_contains_a = range_b_start >= range_a_start && range_b_end <= range_a_end;

        if a_contains_b || b_contains_a {
            contain_count += 1;
        }

        let a_overlaps_b = range_a_end >= range_b_start && range_a_start <= range_b_end;
        let b_overlaps_a = range_b_end >= range_a_start && range_b_start <= range_a_end;

        if a_overlaps_b || b_overlaps_a {
            overlap_count += 1;
        }
    }

    println!("Contained ranges: {contain_count}");
    println!("Overlapped ranges: {overlap_count}");

    Ok(())
}
