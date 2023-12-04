use crate::common::{
    get_number_positions, is_digit, is_neutral, is_symbol, parse_number as common_parse_number,
    NumberLocation,
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const GEAR_SYMBOL: u8 = '*' as u8;

fn get_gear_positions<'a>(line: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
    let mut i = 0;

    std::iter::from_fn(move || {
        while i < line.len() {
            let character = line[i];
            i = i + 1;
            if character == GEAR_SYMBOL {
                return Some(i - 1);
            }
        }

        None
    })
}

#[derive(Clone, Copy)]
struct NumberLocationInLine<'a> {
    number_location: NumberLocation,
    line: &'a [u8],
}

fn parse_number(number_info: NumberLocationInLine) -> u32 {
    common_parse_number(number_info.line, number_info.number_location)
}

fn get_gear_ratio(main_line: &[u8], adjacent_lines: &VecDeque<&[u8]>, gear_index: usize) -> u32 {
    let mut adjacent_numbers: Vec<NumberLocationInLine> = Vec::new();

    for number_location in get_number_positions(main_line) {
        let NumberLocation { index, len } = number_location;
        if index == gear_index + 1 || index + len == gear_index {
            adjacent_numbers.push(NumberLocationInLine {
                number_location,
                line: main_line,
            });
        }
    }

    for line in adjacent_lines {
        for number_location in get_number_positions(line) {
            let NumberLocation { index, len } = number_location;
            if gear_index >= index - 1 && gear_index <= index + len {
                adjacent_numbers.push(NumberLocationInLine {
                    number_location,
                    line,
                });
            }
        }
    }

    if adjacent_numbers.len() != 2 {
        return 0;
    }

    parse_number(adjacent_numbers[0]) * parse_number(adjacent_numbers[1])
}

fn sum_line(line_to_check: &[u8], adjacent_lines: &VecDeque<&[u8]>) -> u32 {
    let mut result: u32 = 0;

    for gear_index in get_gear_positions(line_to_check) {
        let to_add = get_gear_ratio(line_to_check, &adjacent_lines, gear_index);
        result += to_add;
    }

    result
}

pub fn solve(mut lines: Vec<String>) -> u32 {
    let mut current_line: &[u8];
    let mut adjacent_lines: VecDeque<&[u8]> = VecDeque::new();
    let mut result: u32 = 0;

    adjacent_lines.push_front(&lines[0].as_bytes());
    current_line = &lines[1].as_bytes();

    for line in lines.iter().skip(2) {
        adjacent_lines.push_front(line.as_bytes());

        result = result + sum_line(current_line, &adjacent_lines);

        adjacent_lines.pop_back();
        adjacent_lines.push_front(current_line);
        current_line = adjacent_lines.pop_back().unwrap();
    }

    result
}
