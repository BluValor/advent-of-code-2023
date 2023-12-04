pub const NEUTRAL_CHARACTER: u8 = '.' as u8;
pub const THRESHOLD_BOTTOM: u8 = '0' as u8 - 1;
pub const THRESHOLD_TOP: u8 = '9' as u8 + 1;

pub fn is_neutral(c: u8) -> bool {
    c == NEUTRAL_CHARACTER
}

pub fn is_digit(c: u8) -> bool {
    c > THRESHOLD_BOTTOM && c < THRESHOLD_TOP
}

pub fn is_symbol(c: u8) -> bool {
    !is_neutral(c)
}

#[derive(Clone, Copy)]
pub struct NumberLocation {
    pub index: usize,
    pub len: usize,
}

pub fn get_number_positions<'a>(line: &'a [u8]) -> impl Iterator<Item = NumberLocation> + 'a {
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

pub fn parse_number(line: &[u8], number_location: NumberLocation) -> u32 {
    let NumberLocation { index, len } = number_location;
    let number_str = &line[index..(index + len)];
    std::str::from_utf8(&number_str).unwrap().parse().unwrap()
}