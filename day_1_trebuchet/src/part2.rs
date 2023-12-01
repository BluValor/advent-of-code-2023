use std::usize;

const PREFIXES: [&str; 20] = [
  "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
  "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUMBERS: [u32; 20] = [
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
];

pub fn get_line_number(line: &str) -> u32 {
  let first_number_index = (0..line.len())
    .map(|i| &line[i..])
    .map(|substring: &str| PREFIXES.iter().position(|&prefix| substring.starts_with(prefix)))
    .find(|number_string_index: &Option<usize>| {
      match number_string_index {
        None => false,
        Some(_) => true,
      }
    })
    .unwrap()
    .unwrap();
  let first_number = NUMBERS[first_number_index];
  
  let last_number_index = (1..(line.len()+1))
    .rev()
    .map(|i| &line[..i])
    .map(|substring: &str| PREFIXES.iter().position(|&prefix| substring.ends_with(prefix)))
    .find(|number_string_index: &Option<usize>| {
      match number_string_index {
        None => false,
        Some(_) => true,
      }
    })
    .unwrap()
    .unwrap();
  let last_number = NUMBERS[last_number_index];

  format!("{}{}", first_number, last_number).parse().unwrap()
}