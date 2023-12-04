use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod common;
use common::NEUTRAL_CHARACTER;

mod part1;
use part1::solve as part1_solve;

mod part2;
use part2::solve as part2_solve;

fn add_neutral_border(lines: Vec<String>) -> Vec<String> {
    let length = lines[0].len();
    let nautral_character = NEUTRAL_CHARACTER as char;
    let mut lines: Vec<String> = lines.iter().map(|x| format!("{}{}{}", nautral_character, x, nautral_character)).collect();
    let neutral_string: String = std::iter::repeat(nautral_character).take(length + 2).collect();
    lines.insert(0, neutral_string.clone());
    lines.push(neutral_string);
    lines
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<_, _>>()?;
    let lines = add_neutral_border(lines);
    let result = part2_solve(lines);
    println!("{}", result);

    Ok(())
}
