// define a deck type as a collection of cards
use crate::cards::Card;

// Deck is a struct that contains a vector of cards
// it acts as a wrapping on Vec<Card> with needed attributes and methods
#[derive(Debug)]
pub struct Deck {
    card_list: Vec<Card>,
}
