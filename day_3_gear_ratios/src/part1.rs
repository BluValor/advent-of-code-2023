use crate::common::{
    get_number_positions, is_digit, is_neutral, is_symbol, parse_number, NumberLocation,
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_for_symbols(
    main_line: &[u8],
    adjacent_lines: &VecDeque<&[u8]>,
    number_location: NumberLocation,
) -> bool {
    let NumberLocation { index, len } = number_location;

    let column_index = index - 1;
    if is_symbol(main_line[column_index]) {
        return true;
    }
    for line in adjacent_lines {
        if is_symbol(line[column_index]) {
            return true;
        }
    }

    let column_index = index + len;
    if is_symbol(main_line[column_index]) {
        return true;
    }
    for line in adjacent_lines {
        if is_symbol(line[column_index]) {
            return true;
        }
    }

    for line in adjacent_lines {
        for i in (index)..(index + len) {
            if is_symbol(line[i]) {
                return true;
            }
        }
    }

    return false;
}

fn sum_line(line_to_check: &[u8], adjacent_lines: &VecDeque<&[u8]>) -> u32 {
    let mut result: u32 = 0;

    for number_location in get_number_positions(line_to_check) {
        if check_for_symbols(line_to_check, &adjacent_lines, number_location.clone()) {
            let to_add = parse_number(line_to_check, number_location);
            result = result + to_add;
        }
    }

    result
}

pub fn solve(lines: Vec<String>) -> u32 {
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
