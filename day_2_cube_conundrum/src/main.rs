use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod common;
use common::{Draw, Game};

mod part1;
use part1::get_game_number as part1_get_game_number;

mod part2;
use part2::get_game_number as part2_get_game_number;

fn parse_draw(draw_str: &str) -> Draw {
    let mut draw = Draw { r: 0, g: 0, b: 0 };
    draw_str.split(", ").for_each(|color_count_str: &str| {
        let mut color_info = color_count_str.split(' ');
        let (count_str, color_str) = (color_info.next().unwrap(), color_info.next().unwrap());
        match color_str.chars().next().unwrap() {
            'r' => {
                draw.r = count_str.parse::<u32>().unwrap();
            }
            'g' => {
                draw.g = count_str.parse::<u32>().unwrap();
            }
            'b' => {
                draw.b = count_str.parse::<u32>().unwrap();
            }
            _ => {}
        };
    });
    draw
}

fn parse_game(line: &str) -> Game {
    let draws: Vec<Draw> = line.split(": ").collect::<Vec<&str>>()[1]
        .split("; ")
        .map(|draw_str: &str| parse_draw(draw_str))
        .collect();

    Game { draws }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let result = reader.lines().enumerate().fold(0, |acc, (i, line)| {
        let line = line.unwrap();
        let game = parse_game(&line);
        let game_index = (i + 1) as u32;
        // acc + part1_get_game_number(game, game_index)
        acc + part2_get_game_number(game)
    });

    println!("{}", result);

    Ok(())
}
