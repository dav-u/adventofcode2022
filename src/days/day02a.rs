use std::io::{self, BufRead};
use std::fs::File;
use std::convert::From;

enum HandShape {
    Rock, Paper, Scissors
}

struct Round {
    opponent_hand_shape: HandShape,
    my_hand_shape: HandShape,
}

impl From<&str> for Round {
    fn from(item: &str) -> Self {
        let splitted_line = item.split(" ").collect::<Vec<&str>>();
        let opponent_hand_shape = get_opponent_hand_shape(splitted_line[0]);
        let my_hand_shape = get_my_hand_shape(splitted_line[1]);

        Self {
            opponent_hand_shape,
            my_hand_shape,
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
}

impl Round {
    fn get_my_points(&self) -> i32 {
        let game_points = match (&self.opponent_hand_shape, &self.my_hand_shape) {
            (HandShape::Rock, HandShape::Rock) => 3,
            (HandShape::Rock, HandShape::Paper) => 6,
            (HandShape::Rock, HandShape::Scissors) => 0,
            (HandShape::Paper, HandShape::Rock) => 0,
            (HandShape::Paper, HandShape::Paper) => 3,
            (HandShape::Paper, HandShape::Scissors) => 6,
            (HandShape::Scissors, HandShape::Rock) => 6,
            (HandShape::Scissors, HandShape::Paper) => 0,
            (HandShape::Scissors, HandShape::Scissors) => 3,
        };

        game_points + self.my_hand_shape.get_points()
    }
}

pub fn day02_a() {
    let filename = "./src/inputs/day02.txt";
    let file = File::open(filename).unwrap();
    let buf_reader = io::BufReader::new(file);

    //let lines = bufReader.lines().map(|l| l.unwrap());
    let lines = buf_reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let rounds: Vec<Round> = lines.iter().map(|l| l.as_str().into()).collect();
    let my_total_points = rounds.iter().map(|r| r.get_my_points()).sum::<i32>();

    println!("{}", my_total_points);
}

fn get_my_hand_shape(code: &str) -> HandShape {
    match code {
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scissors,
        _ => panic!("Invalid input char for opponent hand shape"),
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
