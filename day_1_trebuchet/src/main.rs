use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod part1;
use part1::get_line_number as part1_get_line_number;

mod part2;
use part2::get_line_number as part2_get_line_number;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .fold(0, |acc, line| acc + part2_get_line_number(&line.unwrap()));

    println!("{}", result);

    Ok(())
}
