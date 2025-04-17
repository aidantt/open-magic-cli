// definition of structs, enums, implementations

pub mod cards;

pub mod mana;

pub mod decks;

// test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_import() {
        // Test the import_from_archidekt function
        let url: &str = "https://archidekt.com/decks/123456";
        let my_deck: decks::Deck = decks::Deck::import_from_archidekt(url);
        // we need to be able to access the deck name, i.e. Archidekt metadata
        assert_eq!(my_deck.name(), "test deck");

        // as well as game-relevant info
        println!("Card List:\n{}", my_deck.list());
        assert_eq!(my_deck.length(), 10);
    }
}
