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

struct EvalTable {
    beats: HashMap<Shape, Shape>,
    beaten_by: HashMap<Shape, Shape>,
}

impl EvalTable {
    fn new() -> EvalTable {
        EvalTable {
            beats: HashMap::from([
                (Shape::Rock, Shape::Scissors),
                (Shape::Paper, Shape::Rock),
                (Shape::Scissors, Shape::Paper),
            ]),
            beaten_by: HashMap::from([
                (Shape::Rock, Shape::Paper),
                (Shape::Paper, Shape::Scissors),
                (Shape::Scissors, Shape::Rock),
            ]),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Strat {
    Win,
    Lose,
    Draw,
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

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines().map(|l| l.unwrap());

    let eval_table = EvalTable::new();

    let mut scores = [0; 2];

    for line in lines {
        let tokens: Vec<&str> = line.trim().split(' ').collect();

        if tokens.len() != 2 {
            panic!("Malformed input: {:?}", tokens);
        }

        let player_strat = parse_strategy(tokens[1]);
        let op_shape = parse_shape(tokens[0]);
        let player_shape = choose_shape_from_strat(&eval_table, op_shape, player_strat);

        let (score_a, score_b) = eval_round(&eval_table, op_shape, player_shape);

        scores[0] += score_a;
        scores[1] += score_b;
    }

    println!("The total score of the opponent is: {:?}", scores[0]);
    println!("The total score of the player is: {:?}", scores[1]);

    Ok(())
}

fn parse_shape(shape_key: &str) -> Shape {
    match shape_key {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("{shape_key} does not correspond to a shape."),
    }
}

fn parse_strategy(strat_key: &str) -> Strat {
    match strat_key {
        "X" => Strat::Lose,
        "Y" => Strat::Draw,
        "Z" => Strat::Win,
        _ => panic!("{strat_key} does not correspond to a strategy."),
    }
}

fn choose_shape_from_strat(table: &EvalTable, op_shape: Shape, player_strat: Strat) -> Shape {
    match player_strat {
        Strat::Win => *table.beaten_by.get(&op_shape).unwrap(),
        Strat::Lose => *table.beats.get(&op_shape).unwrap(),
        Strat::Draw => op_shape,
    }
}

fn eval_round(table: &EvalTable, shape_a: Shape, shape_b: Shape) -> (i32, i32) {
    if shape_a == shape_b {
        (
            DRAW_SCORE + shape_a.get_score(),
            DRAW_SCORE + shape_b.get_score(),
        )
    } else if *table.beats.get(&shape_a).unwrap() == shape_b {
        (
            WIN_SCORE + shape_a.get_score(),
            LOSS_SCORE + shape_b.get_score(),
        )
    } else {
        (
            LOSS_SCORE + shape_a.get_score(),
            WIN_SCORE + shape_b.get_score(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw() {
        let t = EvalTable::new();

        let res = eval_round(&t, Shape::Rock, Shape::Rock);
        assert_eq!(res.0, ROCK_SCORE + DRAW_SCORE);
        assert_eq!(res.1, ROCK_SCORE + DRAW_SCORE);
    }
    #[test]
    fn win() {
        let t = EvalTable::new();

        let res = eval_round(&t, Shape::Rock, Shape::Scissors);
        assert_eq!(res.0, ROCK_SCORE + WIN_SCORE);
        assert_eq!(res.1, SCISSORS_SCORE + LOSS_SCORE);
    }

    #[test]
    fn lose() {
        let t = EvalTable::new();

        let res = eval_round(&t, Shape::Paper, Shape::Scissors);
        assert_eq!(res.0, PAPER_SCORE + LOSS_SCORE);
        assert_eq!(res.1, SCISSORS_SCORE + WIN_SCORE);
    }

    #[test]
    fn choose_draw() {
        let t = EvalTable::new();

        let res = choose_shape_from_strat(&t, Shape::Rock, Strat::Draw);
        assert_eq!(res, Shape::Rock);
    }

    #[test]
    fn choose_win() {
        let t = EvalTable::new();

        let res = choose_shape_from_strat(&t, Shape::Paper, Strat::Win);
        assert_eq!(res, Shape::Scissors);
    }

    #[test]
    fn choose_lose() {
        let t = EvalTable::new();

        let res = choose_shape_from_strat(&t, Shape::Scissors, Strat::Lose);
        assert_eq!(res, Shape::Paper);
    }
}
