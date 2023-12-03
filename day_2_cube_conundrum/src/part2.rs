use crate::common::{Draw, Game};

pub fn get_game_number(game: Game) -> u32 {
    let mut min_draw = Draw { r: 0, g: 0, b: 0 };
    game.draws.iter().for_each(|draw| {
        min_draw.r = u32::max(min_draw.r, draw.r);
        min_draw.g = u32::max(min_draw.g, draw.g);
        min_draw.b = u32::max(min_draw.b, draw.b);
    });
    min_draw.r * min_draw.g * min_draw.b
}
