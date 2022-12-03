use std::io::BufRead;
use std::collections::HashSet;

pub fn day03_a() {
    let filename = "./src/inputs/day03.txt";
    let file = std::fs::File::open(filename).unwrap();
    let buf_reader = std::io::BufReader::new(file);

    let lines = buf_reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut sum = 0;

    for line in lines {
        let first_compartment = &line[..line.len()/2];
        let second_compartment = &line[line.len()/2..];

        let first_compartment_set: HashSet<char> = first_compartment.chars().collect();
        let second_compartment_set: HashSet<char> = second_compartment.chars().collect();

        let duplicate = *first_compartment_set
            .intersection(&second_compartment_set)
            .next()
            .unwrap();

        let value = if duplicate.is_ascii_lowercase() { duplicate as u32 - 'a' as u32 + 1 }
                         else { duplicate as u32 - 'A' as u32 + 27 };

        sum += value;
    }

    println!("{}", sum);
}

pub fn day03_b() {
    let filename = "./src/inputs/day03.txt";
    let file = std::fs::File::open(filename).unwrap();
    let buf_reader = std::io::BufReader::new(file);

    let lines = buf_reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let chunks = lines.chunks(3);

    let mut sum = 0;

    for chunk in chunks {
        let mut sets = chunk
            .iter()
            .map(|l| l.chars().collect::<HashSet<char>>());
        
        let first_set = sets.next().unwrap();
        let duplicate = *sets
            .fold(first_set, |s1, s2| &s1 & &s2)
            .iter()
            .next()
            .unwrap();

        let value = if duplicate.is_ascii_lowercase() { duplicate as u32 - 'a' as u32 + 1 }
                         else { duplicate as u32 - 'A' as u32 + 27 };

        sum += value;
    }

    println!("{}", sum);
}