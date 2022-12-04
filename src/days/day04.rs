use std::io::BufRead;

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_with(&self, other: &Section) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

impl From<&str> for Section {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once("-").unwrap();

        Self {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        }
    }
}

struct ElfPair {
    first_section: Section,
    second_section: Section,
}

impl ElfPair {
    fn is_one_unnecessary(&self) -> bool {
        self.first_section.contains(&self.second_section)
        || self.second_section.contains(&self.first_section)
    }
}

impl From<&str> for ElfPair {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(",").unwrap();

        Self { first_section: left.into(), second_section: right.into() }
    }
}

pub fn day04_a() {
    let filename = "./src/inputs/day04.txt";
    let file = std::fs::File::open(filename).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    let lines: Result<Vec<_>, _> = buf_reader.lines().collect();
    let lines = lines.unwrap();

    let elf_pairs: Vec<ElfPair> = lines.iter().map(|l| l.as_str().into()).collect();
    let count = elf_pairs.iter().filter(|p| p.is_one_unnecessary()).count();

    println!("{}", count);
}

pub fn day04_b() {
    let filename = "./src/inputs/day04.txt";
    let file = std::fs::File::open(filename).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    let lines: Result<Vec<_>, _> = buf_reader.lines().collect();
    let lines = lines.unwrap();

    let elf_pairs: Vec<ElfPair> = lines.iter().map(|l| l.as_str().into()).collect();
    let count = elf_pairs.iter().filter(|p| p.first_section.overlaps_with(&p.second_section)).count();

    println!("{}", count);
}