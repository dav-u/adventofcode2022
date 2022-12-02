use std::io::{self, BufRead};
use std::fs::File;
use std::convert::From;

#[derive(Clone, Copy)]
enum HandShape {
    Rock, Paper, Scissors
}

enum RoundResult {
    Win, Lose, Tie
}

struct Round {
    opponent_hand_shape: HandShape,
    result: RoundResult,
}

impl From<&str> for Round {
    fn from(item: &str) -> Self {
        let splitted_line = item.split(" ").collect::<Vec<&str>>();
        let opponent_hand_shape = get_opponent_hand_shape(splitted_line[0]);
        let round_result = get_round_result(splitted_line[1]);

        Self {
            opponent_hand_shape,
            result: round_result,
        }
    }
}

impl RoundResult {
    fn get_points(&self) -> i32 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Tie => 3,
            RoundResult::Win => 6,
        }
    }
}

impl HandShape {
    fn get_points(&self) -> i32 {
        match &self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn wins_against(&self) -> HandShape {
        match &self {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
        }
    }

    fn loses_against(&self) -> HandShape {
        match &self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        }
    }
}

impl Round {
    fn get_my_hand_shape(&self) -> HandShape {
        match self.result {
            RoundResult::Tie => self.opponent_hand_shape,
            RoundResult::Win => self.opponent_hand_shape.loses_against(),
            RoundResult::Lose => self.opponent_hand_shape.wins_against(),
        }
    }

    fn get_my_points(&self) -> i32 {
        self.result.get_points() + self.get_my_hand_shape().get_points()
    }
}

pub fn day02_b() {
    let filename = "./src/inputs/day02.txt";
    let file = File::open(filename).unwrap();
    let buf_reader = io::BufReader::new(file);

    let lines = buf_reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let rounds: Vec<Round> = lines.iter().map(|l| l.as_str().into()).collect();
    let my_total_points = rounds.iter().map(|r| r.get_my_points()).sum::<i32>();

    println!("{}", my_total_points);
}

fn get_round_result(code: &str) -> RoundResult {
    match code {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Tie,
        "Z" => RoundResult::Win,
        _ => panic!("Invalid input char for round result"),
    }
}

fn get_opponent_hand_shape(code: &str) -> HandShape {
    match code {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!("Invalid input char for opponent hand shape"),
    }
}
