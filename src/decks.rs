// define a deck type as a collection of cards
use crate::cards::Card;

// Deck is a struct that contains a vector of cards
// it acts as a wrapping on Vec<Card> with needed attributes and methods
pub struct Deck {
    card_list: Vec<Card>,
}

impl Deck {
    // construct a deck object by resolving an archidekt url
    pub fn import_from_archidekt(url: &str) -> Self {
        // Archidekt has api that returns all deck information in JSON format
        // based on the deck id, extractable from the deck page url
        // 1. get deck id from url
        // (further implementation: check for a cached yaml here and use that
        //  instead of querying if possible to minimize queries)
        // 2. use deck id and a web query crate to query the Archidekt api
        // 3. handle query errors
        // 4. parse successful query as a YAML 
        // 5. cache deck YAML and assemble Deck object 
        //    (lots of specifics to iron out here)
    }
}
