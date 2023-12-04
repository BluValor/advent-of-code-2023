use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod common;
use common::Card;

mod part1;
use part1::get_points as part1_get_points;

mod part2;
use part2::get_points as part2_get_points;

fn parse_line(line: &str) -> Card {
    let mut line_split = line.split(": ");
    let index: u32 = line_split.next().unwrap()[5..].trim().parse().unwrap();
    let mut games_split = line_split.next().unwrap().split(" | ");
    let winning: Vec<u32> = games_split
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|number_str| number_str.parse().unwrap())
        .collect();
    let guessed: Vec<u32> = games_split
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|number_str| number_str.parse().unwrap())
        .collect();
    Card {
        index,
        winning,
        guessed,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let cards: Vec<Card> = reader.lines().map(|line| parse_line(&line.unwrap())).collect();
    // let points = part1_get_points(&cards);
    let points = part2_get_points(&cards);

    println!("{}", points);

    Ok(())
}
