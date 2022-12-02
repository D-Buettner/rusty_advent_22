use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSS_SCORE: i32 = 0;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

type EvalTable = HashMap<Shape, Shape>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    // Returns the point value for this shape.
    fn get_score(&self) -> i32 {
        match *self {
            Shape::Rock => ROCK_SCORE,
            Shape::Paper => PAPER_SCORE,
            Shape::Scissors => SCISSORS_SCORE,
        }
    }
}

// struct StrategyLine<'a>(&'a str, &'a str);

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines().map(|l| l.unwrap());

    let eval_table: EvalTable = HashMap::from([
        (Shape::Rock, Shape::Scissors),
        (Shape::Paper, Shape::Rock),
        (Shape::Scissors, Shape::Paper),
    ]);

    let mut scores = [0; 2];

    for line in lines {
        let plays: Vec<&str> = line.trim().split(' ').collect();

        if plays.len() != 2 {
            panic!("Malformed input: {:?}", plays);
        }

        let (score_a, score_b) =
            eval_round(&eval_table, parse_shape(plays[0]), parse_shape(plays[1]));

        scores[0] += score_a;
        scores[1] += score_b;
    }

    println!("The total score of the opponent is: {:?}", scores[0]);
    println!("The total score of the player is: {:?}", scores[1]);

    Ok(())
}

fn parse_shape(shape_key: &str) -> Shape {
    match shape_key {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("{shape_key} does not correspond to a shape."),
    }
}

fn eval_round(table: &EvalTable, hand_a: Shape, hand_b: Shape) -> (i32, i32) {
    if hand_a == hand_b {
        (
            DRAW_SCORE + hand_a.get_score(),
            DRAW_SCORE + hand_b.get_score(),
        )
    } else if *table.get(&hand_a).unwrap() == hand_b {
        (
            WIN_SCORE + hand_a.get_score(),
            LOSS_SCORE + hand_b.get_score(),
        )
    } else {
        (
            LOSS_SCORE + hand_a.get_score(),
            WIN_SCORE + hand_b.get_score(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw() {
        let t = HashMap::from([
            (Shape::Rock, Shape::Scissors),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Paper),
        ]);

        let res = eval_round(&t, Shape::Rock, Shape::Rock);
        assert_eq!(res.0, ROCK_SCORE + DRAW_SCORE);
        assert_eq!(res.1, ROCK_SCORE + DRAW_SCORE);
    }
    #[test]
    fn win() {
        let t = HashMap::from([
            (Shape::Rock, Shape::Scissors),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Paper),
        ]);

        let res = eval_round(&t, Shape::Rock, Shape::Scissors);
        assert_eq!(res.0, ROCK_SCORE + WIN_SCORE);
        assert_eq!(res.1, SCISSORS_SCORE + LOSS_SCORE);
    }

    #[test]
    fn lose() {
        let t = HashMap::from([
            (Shape::Rock, Shape::Scissors),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Paper),
        ]);

        let res = eval_round(&t, Shape::Paper, Shape::Scissors);
        assert_eq!(res.0, PAPER_SCORE + LOSS_SCORE);
        assert_eq!(res.1, SCISSORS_SCORE + WIN_SCORE);
    }
}
