use std::fmt::Display;

use rand::Rng;

struct Card {
    suit: Suit,
    value: i32,
}

struct Deck(Vec<Card>);

#[derive(Clone, Copy)]
enum Suit {
    Heart, Club, Diamond, Spade,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::<Card>::with_capacity(52);
        cards.append(&mut Deck::build_suit(Suit::Heart));
        cards.append(&mut Deck::build_suit(Suit::Diamond));
        cards.append(&mut Deck::build_suit(Suit::Club));
        cards.append(&mut Deck::build_suit(Suit::Spade));
        Deck(cards)
    }

    fn build_suit(suit: Suit) -> Vec<Card> {
        let mut cards = Vec::<Card>::with_capacity(13);
        for n in 1..cards.capacity() {
            cards.push(Card {
                suit,
                value: n as i32,
            });
        }
        cards
    }
    fn shuffle(&mut self, nb_permutations: i32) {
        let mut rng = rand::thread_rng();
        for _ in 0..nb_permutations {
            let first_random_index = rng.gen_range(0..self.0.len());
            let second_random_index = rng.gen_range(0..self.0.len());
            self.0.swap(first_random_index, second_random_index);
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            Self::Heart => String::from("♥️"),
            Self::Diamond => String::from("♦️"),
            Self::Club => String::from("♣️"),
            Self::Spade => String::from("♠️"),
        };
        write!(f, "{}", emoji)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            other_value => other_value.to_string(),
        };
        write!(f, "{}{}", value, self.suit)
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for single_card in self.0.iter() {
            write!(f, "{} ", single_card)?;
        }
        Ok(())
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Initital deck = \n{}\n", deck);

    deck.shuffle(5);
    println!("After 5 permutations = \n{}\n", deck);

    deck.shuffle(150);
    println!("After 150 permutations = \n{}\n", deck);
}
