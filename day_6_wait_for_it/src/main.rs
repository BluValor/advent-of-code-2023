use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

fn part1_parse_input(lines: Vec<String>) -> Vec<Race> {
    let times: Vec<u64> = lines[0][5..]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let records: Vec<u64> = lines[1][9..]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    times
        .iter()
        .cloned()
        .zip(records.iter().cloned())
        .map(|(time, record)| Race { time, record })
        .collect()
}

fn part2_parse_input(lines: Vec<String>) -> Race {
    let time: u64 = lines[0][5..].chars().filter(|&c| !(c == ' ')).collect::<String>().parse().unwrap();
    let record: u64 = lines[1][9..].chars().filter(|&c| !(c == ' ')).collect::<String>().parse().unwrap();
    Race { time, record }
}

/*
    The function for the maximum possible time is the maximum of a quadratic function f(x) = x(x - n) where:
        - x is the tima at which the button was released, and so the speed at which the boat will move
        - n is the maximum possible time
    The current record is a constant function f(x) = r where:
        - r is the current record
    The points at which these two two functions intersect are given by equation x(n - x) = r.
    This results in a quadratic function f(x) = -x^2 + nx - r
    The points at which this function is 0 are the values of x (time) at which releasing the button would result in a distance tied to the current record. Everything between is a new record.
    These points are given by functions:
        d = n^2 - 4(-1)(-r) = n^2 - 4r
        x1 = (-n + sqrt(d)) / (-2) = (n - sqrt(d)) / 2
        x2 = (-n - sqrt(d)) / (-2) = (n + sqrt(d)) / 2
        In this case x1 < x2.
    The solution is to calculate these values and calcualte the number of integers between them, which would give us the number of possibilities to win the current race.
*/
fn get_number_of_possibilities(race: &Race) -> u64 {
    let n = race.time as f64;
    let r = race.record as f64;
    let d: f64 = n.powi(2) - (4.0 * r);
    let x1: f64 = (n - d.sqrt()) / 2.0;
    let x2: f64 = (n + d.sqrt()) / 2.0;
    let mut result = ((x1.ceil() as u64)..=(x2.floor() as u64))
        .into_iter()
        .count() as u64;
    let is_x1_int = x1.fract() == 0.0;
    let is_x2_int = x2.fract() == 0.0;
    if is_x1_int {
        result -= 1;
    }
    if is_x2_int {
        result -= 1;
    }
    result
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;

    // let result = part1_parse_input(lines)
    let result = vec![part2_parse_input(lines)]
        .iter()
        .map(|race| get_number_of_possibilities(race))
        .fold(1, |acc, x| acc * x);
    println!("{}", result);

    Ok(())
}
