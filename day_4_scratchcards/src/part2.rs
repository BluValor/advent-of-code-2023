use crate::common::{Card, get_card_intersection_size};

pub fn get_points(cards: &Vec<Card>) -> u32 {
    let mut copies: Vec<u32> = vec![1; cards.len()];
    for card in cards {
        let winnings = get_card_intersection_size(card);
        let multiplier = copies[(card.index - 1) as usize];
        for i in (card.index)..(card.index + winnings) {
            copies[i as usize] += multiplier;
        }
    }
    copies.iter().sum()
}