use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Card {
    Card2,
    Card3,
    Card4, 
    Card5, 
    Card6, 
    Card7, 
    Card8, 
    Card9, 
    CardT,
    CardJ, 
    CardQ, 
    CardK, 
    CardA, 
}

impl Card {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Self::Card2),
            '3' => Some(Self::Card3),
            '4' => Some(Self::Card4),
            '5' => Some(Self::Card5),
            '6' => Some(Self::Card6),
            '7' => Some(Self::Card7),
            '8' => Some(Self::Card8),
            '9' => Some(Self::Card9),
            'T' => Some(Self::CardT),
            'J' => Some(Self::CardJ),
            'Q' => Some(Self::CardQ),
            'K' => Some(Self::CardK),
            'A' => Some(Self::CardA),
            _   => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl HandType {
    pub fn from_cards(cards: &Vec<Card>) -> Self {
        let mut counts: HashMap<Card, i32> = HashMap::new();
        for &c in cards {
            let entry = counts.entry(c);
            let counter = entry.or_insert(0);
            *counter += 1;
        }

        let mut pair = false;
        let mut three = false;
        for (key, value) in counts.iter() {
            match value {
                5 => { return Self::Five; },
                4 => { return Self::Four; },
                3 => { 
                    if pair {
                        return Self::Full;
                    }
                    three = true;
                },
                2 => {
                    if pair {
                        return Self::TwoPair;
                    }
                    if three {
                        return Self::Full;
                    }
                    pair = true;
                },
                _ => {},
            }
        }

        if pair {
            return Self::OnePair;
        }

        if three {
            return Self::Three;
        }

        return Self::HighCard;
    }
}

#[derive(Debug)]
pub struct Hand<'a> {
    pub text: &'a str,
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bet: u32,
}

impl Hand<'_> {
    pub fn from_text(text: &str) -> Hand {
        let cards_and_bet: Vec<_> = text.split(' ').collect();
        let cards_str = cards_and_bet[0];
        let cards: Vec<Card> = cards_str.chars().map(|c| Card::from_char(c).unwrap()).collect();
        let hand_type = HandType::from_cards(&cards);

        Hand {
            text: cards_str,
            cards,
            hand_type,
            bet: cards_and_bet[1].parse().unwrap(),
        }
    }
}

pub fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    match h1.hand_type.cmp(&h2.hand_type) {
        Ordering::Greater => { return Ordering::Greater; },
        Ordering::Less    => { return Ordering:: Less; },
        _                 => {
            if let Some((c1, c2)) = h1.cards.iter().zip(h2.cards.iter()).find(|(a, b)| a.cmp(&b) != Ordering::Equal) {
                return c1.cmp(&c2);
            }
            return Ordering::Equal;
        }
    }
}