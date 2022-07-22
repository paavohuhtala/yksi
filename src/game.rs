use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

pub const COLORS: [Color; 4] = [Color::Red, Color::Yellow, Color::Green, Color::Blue];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Card {
    Number(Color, u8),
    Skip(Color),
    Reverse(Color),
    DrawTwo(Color),
    Wild,
    WildDrawFour,
}

impl Card {
    pub fn color(&self) -> Option<Card> {
        match self {
            Card::Number(color, _) => Some(Card::Number(*color, 0)),
            Card::Skip(color) => Some(Card::Skip(*color)),
            Card::Reverse(color) => Some(Card::Reverse(*color)),
            Card::DrawTwo(color) => Some(Card::DrawTwo(*color)),
            Card::Wild => None,
            Card::WildDrawFour => None,
        }
    }
}

struct Table {
    deck: Vec<Card>,
    discard: Vec<Card>,
}

struct Player {
    cards: Vec<Card>,
}

pub struct Game {
    table: Table,
    players: Vec<Player>,
    current_player: usize,
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for &color in COLORS.iter() {
        deck.push(Card::Number(color, 0));

        for number in 1..=9 {
            deck.push(Card::Number(color, number));
            deck.push(Card::Number(color, number));
        }

        deck.push(Card::Skip(color));
        deck.push(Card::Skip(color));

        deck.push(Card::Reverse(color));
        deck.push(Card::Reverse(color));

        deck.push(Card::DrawTwo(color));
        deck.push(Card::DrawTwo(color));
    }

    for i in 0..4 {
        deck.push(Card::Wild);
        deck.push(Card::WildDrawFour);
    }

    deck
}

pub fn create_game() -> Game {
    let mut deck = create_deck();
    deck.shuffle(&mut rand::thread_rng());

    Game {
        current_player: 0,
        players: vec![Player { cards: Vec::new() }],
        table: Table {
            deck,
            discard: Vec::new(),
        },
    }
}
