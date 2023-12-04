#[derive(Debug)]
pub struct Card {
    pub index: u32,
    pub winning: Vec<u32>,
    pub guessed: Vec<u32>,
}

pub fn get_card_intersection_size(card: &Card) -> u32 {
    let mut intersection_size = 0;
    for guess in &card.guessed {
        if card.winning.contains(&guess) {
            intersection_size += 1;
        }
    }
    intersection_size
}