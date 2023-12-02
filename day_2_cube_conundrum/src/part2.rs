use crate::common::{Draw, Game};

pub fn get_game_number(game: Game, game_index: u32) -> u32 {
    let mut min_draw = Draw { r: 0, g: 0, b: 0 };
    game.draws.iter().for_each(|draw| {
        if min_draw.r < draw.r {
            min_draw.r = draw.r;
        }
        if min_draw.g < draw.g {
            min_draw.g = draw.g;
        }
        if min_draw.b < draw.b {
            min_draw.b = draw.b;
        }
    });
    min_draw.r * min_draw.g * min_draw.b
}
