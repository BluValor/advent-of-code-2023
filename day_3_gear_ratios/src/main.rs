use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const NEUTRAL_CHARACTER: u8 = '.' as u8;
const THRESHOLD_BOTTOM: u8 = '0' as u8 - 1;
const THRESHOLD_TOP: u8 = '9' as u8 + 1;

fn is_neutral(c: u8) -> bool {
    c == NEUTRAL_CHARACTER
}

fn is_digit(c: u8) -> bool {
    c > THRESHOLD_BOTTOM && c < THRESHOLD_TOP
}

fn is_symbol(c: u8) -> bool {
    !is_neutral(c)
}

#[derive(Clone)]
struct NumberLocation {
    index: usize,
    len: usize,
}

fn get_number_positions<'a>(line: &'a [u8]) -> impl Iterator<Item = NumberLocation> + 'a {
    let mut i = 0;

    std::iter::from_fn(move || {
        let mut number_index: usize = 0;
        let mut number_length: usize = 0;
        let mut parsing = false;

        while i < line.len() {
            let character = line[i];
            i = i + 1;

            if is_digit(character) {
                if parsing {
                    number_length = number_length + 1;
                } else {
                    parsing = true;
                    number_index = i - 1;
                    number_length = 1;
                }
            } else {
                if parsing {
                    return Some(NumberLocation {
                        index: number_index,
                        len: number_length,
                    });
                }
            }
        }

        if parsing {
            return Some(NumberLocation {
                index: number_index,
                len: number_length,
            });
        }

        None
    })
}

fn check_for_symbols(
    main_line: &[u8],
    adjacent_lines: &VecDeque<&[u8]>,
    number_location: NumberLocation,
) -> bool {
    let NumberLocation { index, len } = number_location;

    if index > 0 {
        let column_index = index - 1;
        if is_symbol(main_line[column_index]) {
            return true;
        }
        for line in adjacent_lines {
            if is_symbol(line[column_index]) {
                return true;
            }
        }
    }

    if index + len < main_line.len() {
        let column_index = index + len;
        if is_symbol(main_line[column_index]) {
            return true;
        }
        for line in adjacent_lines {
            if is_symbol(line[column_index]) {
                return true;
            }
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

fn parse_number(line: &[u8], number_location: NumberLocation) -> u32 {
    let NumberLocation { index, len } = number_location;
    let number_str = &line[index..(index + len)];
    std::str::from_utf8(&number_str).unwrap().parse().unwrap()
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

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut current_line: &[u8];
    let mut adjacent_lines: VecDeque<&[u8]> = VecDeque::new();
    let mut result: u32;

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    current_line = &lines[0].as_bytes();
    adjacent_lines.push_front(&lines[1].as_bytes());
    result = sum_line(current_line, &adjacent_lines);

    adjacent_lines.push_front(current_line);
    current_line = adjacent_lines.pop_back().unwrap();

    let mut counter = 2;

    for line in lines.iter().skip(2) {
        counter = counter + 1;
        adjacent_lines.push_front(line.as_bytes());

        result = result + sum_line(current_line, &adjacent_lines);

        adjacent_lines.pop_back();
        adjacent_lines.push_front(current_line);
        current_line = adjacent_lines.pop_back().unwrap();
    }

    result = result + sum_line(current_line, &adjacent_lines);

    println!("{}", result);

    Ok(())
}
