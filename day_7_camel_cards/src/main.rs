use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod common;
use common::*;

fn calculate_winnings(sorted_hands: &Vec<Hand>) -> u32 {
    sorted_hands.iter().enumerate().fold(0, |acc, (i, hand)| acc + ((i as u32 + 1) * hand.bet))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    let mut hands: Vec<_> = lines.iter().map(|line| Hand::from_text(line)).collect();
    hands.sort_by(|h1, h2| compare_hands(h1, h2));
    let result = calculate_winnings(&hands);
    println!("{}", result);

    Ok(())
}
