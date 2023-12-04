use crate::common::{Card, get_card_intersection_size};

fn get_card_points(card: &Card) -> u32 {
    let intersection_size = get_card_intersection_size(card);
    if intersection_size == 0 {
        return 0;
    } else {
        return 2_u32.pow(intersection_size - 1);
    }
}

pub fn get_points(cards: &Vec<Card>) -> u32 {
    cards.iter().fold(0, |acc, card| acc + get_card_points(card))
}