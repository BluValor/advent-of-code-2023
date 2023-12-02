use crate::common::{Draw, Game};

const DEFAULT_DRAW: Draw = Draw {
    r: 12,
    g: 13,
    b: 14,
};

pub fn get_game_number(game: Game, game_index: u32) -> u32 {
    let is_game_possible = game.draws.iter().all(|draw| DEFAULT_DRAW.contains(draw));
    let result = if is_game_possible { game_index } else { 0 };
    result
}
