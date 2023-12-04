use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Card {
    index: u32,
    winning: Vec<u32>,
    guessed: Vec<u32>,
}

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

fn get_points(card: Card) -> u32 {
    let mut intersection_size = 0;
    for guess in card.guessed {
        if card.winning.contains(&guess) {
            intersection_size += 1;
        }
    }
    if intersection_size == 0 {
        return 0;
    } else {
        return 2_u32.pow(intersection_size - 1);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let cards = reader.lines().map(|line| parse_line(&line.unwrap()));
    let points = cards.fold(0, |acc, card| acc + get_points(card));

    println!("{}", points);

    Ok(())
}
