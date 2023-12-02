const THRESHOLD: u32 = '9' as u32 + 1;

// just a helper for engineering the solution
macro_rules! print_char {
    ($ch:literal) => {
        println!(" ___ {}", $ch);
        println!(
            "(u32: {}, i32: {}) -> {}",
            $ch as u32,
            $ch as i32,
            $ch as i32 - THRESHOLD as i32
        );
        println!("is number: {}", ($ch as u32) < (THRESHOLD as u32));
        println!(
            "(u8: {}, i8: {}) -> {}",
            $ch as u8,
            $ch as i8,
            $ch as i8 - THRESHOLD as i8
        );
        println!("is number: {}", ($ch as u8) < (THRESHOLD as u8));
        println!("");
    };
}

// just a helper for engineering the solution
#[allow(dead_code)]
fn check_unicode() {
    print_char!('0');
    print_char!('9');
    print_char!('a');
    print_char!('z');
    print_char!('A');
    print_char!('Z');
}

fn is_number(c: char) -> bool {
    (c as u32) < THRESHOLD
}

pub fn get_line_number(line: &str) -> u32 {
    let first_number = line.chars().find(|&c| is_number(c)).unwrap();
    let last_number = line.chars().rev().find(|&c| is_number(c)).unwrap();
    format!("{}{}", first_number, last_number).parse().unwrap()
}
