use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

use super::Card;

const SUITS: [u8; 4] = [0, 1, 2, 3];
const NUMBERS: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const CARDS_DRAW: usize = 3;
#[derive(Default, Debug)]

pub struct Deck {
    pub deck: Vec<Card>,
    pub fliped_card: Option<Card>,
}
impl Deck {
    pub fn populate(&mut self) {
        let card_values_table: HashMap<u8, u8> = HashMap::from([
            (3, 1),
            (2, 2),
            (1, 3),
            (10, 4),
            (9, 5),
            (8, 6),
            (7, 7),
            (6, 8),
            (5, 9),
            (4, 10),
        ]);
        for suit in SUITS.into_iter() {
            for number in NUMBERS.into_iter() {
                self.deck.push(Card {
                    suit,
                    number,
                    rank: *card_values_table.get(&number).unwrap(),
                    is_manilha: false,
                })
            }
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
    pub fn draw_manilha_setter(&mut self) -> Result<Card, String> {
        let mut rng = thread_rng();
        match self.deck.choose(&mut rng) {
            Some(card_ref) => {
                let card_deref = *card_ref;
                self.deck.retain(|&d| d != card_deref);
                Ok(card_deref)
            }
            None => Err(String::from("Error at drawing manilha setter")),
        }
    }
    pub fn set_manilhas(&mut self, manilha_setter: u8) {
        for card in &mut self.deck {
            if card.number == manilha_setter {
                card.is_manilha = true;
            }
        }
    }
    pub fn draw_cards(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = vec![];
        for _ in 0..CARDS_DRAW {
            hand.push(self.deck.pop().unwrap())
        }
        self.shuffle();
        hand
    }
    pub fn deck_setup(&mut self) {
        self.populate();
        self.shuffle();
        self.fliped_card = Some(self.draw_manilha_setter().unwrap());
        let manilha_setter = (self.fliped_card.unwrap().number + 1) % 11;
        self.set_manilhas(manilha_setter);
    }
    pub fn refresh_deck(&mut self) {
        self.deck.clear();
        self.deck_setup();
    }
}
