use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    len: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_start + self.len
    }

    fn map(&self, value: u64) -> u64 {
        self.destination_start + (value - self.source_start)
    }
}

#[derive(Debug)]
struct Mapping<'a> {
    _source: &'a str,
    destination: &'a str,
    ranges: Vec<Range>,
}

impl Mapping<'_> {
    fn map(&self, value: u64) -> u64 {
        let range = self.ranges.iter().find(|x| x.contains(value));
        match range {
            Some(range) => range.map(value),
            None => value,
        }
    }
}

fn parse_mapping(text: &str) -> (&str, Mapping) {
    let relation_values: Vec<&str> = text.split(" map:\n").collect();
    let source_destination: Vec<&str> = relation_values[0].split("-to-").collect();
    let source = source_destination[0];
    let destination = source_destination[1];
    let ranges: Vec<Range> = relation_values[1]
        .split('\n')
        .map(|values_str| {
            let values: Vec<u64> = values_str
                .split(' ')
                .map(|value_str| value_str.parse().unwrap())
                .collect();
            Range {
                destination_start: values[0],
                source_start: values[1],
                len: values[2],
            }
        })
        .collect();

    (
        source,
        Mapping {
            _source: source,
            destination,
            ranges,
        },
    )
}

fn parse_seed_numbers(line: &str) -> Vec<u64> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|number_str| number_str.parse().unwrap())
        .collect()
}

const DEFAUL_DESCRIPTOR: &str = "seed";

fn map_seed(seed_number: u64, mappings: &HashMap<&str, Mapping>) -> u64 {
    let mut current_key: &str = DEFAUL_DESCRIPTOR;
    let mut current_number: u64 = seed_number;

    while let Some(mapping) = mappings.get(current_key) {
        current_key = mapping.destination;
        current_number = mapping.map(current_number);
    }

    current_number
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let mut numbers_mappings_split = file_content.split("\n\n");

    let seed_numbers = parse_seed_numbers(&numbers_mappings_split.next().unwrap());
    let mappings: HashMap<&str, Mapping> = numbers_mappings_split
        .map(|text| parse_mapping(text))
        .collect();

    let results: Vec<u64> = seed_numbers
        .iter()
        .map(|x| map_seed(*x, &mappings))
        .collect();
    let result: u64 = results.iter().cloned().min().unwrap();
    println!("{}", result);

    Ok(())
}
