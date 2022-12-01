use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

pub fn day01_a() {
    //let filename = "./src/inputs/example_input1.txt";
    let filename = "./src/inputs/day01.txt";
    let file = File::open(filename).unwrap();

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let unfiltered_groups = lines.into_iter().group_by(|l| *l != "");
    let groups = unfiltered_groups.into_iter().filter(|(key, _)| *key);

    let sums = groups.map(|(_, group)| group.map(|s| s.parse::<i32>().unwrap()).sum::<i32>());
    let max = sums.into_iter().max();

    println!("{}", max.unwrap());
}

pub fn day01_b() {
    //let filename = "./src/inputs/day01_example.txt";
    let filename = "./src/inputs/day01.txt";
    let file = File::open(filename).unwrap();

    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let unfiltered_groups = lines.into_iter().group_by(|l| *l != "");
    let groups = unfiltered_groups.into_iter().filter(|(key, _)| *key);

    let mut sums: Vec<i32> = groups.map(|(_, group)| group.map(|s| s.parse::<i32>().unwrap()).sum::<i32>()).collect();

    sums.sort();
    sums.reverse();

    let result = sums.into_iter().take(3).sum::<i32>();

    println!("{}", result);
}
