// open magic cli, an attempt at magic in the terminal

// define a Mana struct to hold values for each type of mana
#[derive(Debug)]
struct Mana {
    white: u32,
    blue: u32,
    black: u32,
    red: u32,
    green: u32,
    colorless: u32,
}

impl Mana {
    // Mana constructor
    fn new(
        white: u32,
        blue: u32,
        black: u32,
        red: u32,
        green: u32,
        colorless: u32,
    ) -> Mana {
        Mana {
            white,
            blue,
            black,
            red,
            green,
            colorless,
        }
    }

    // create a constructor for Mana with all values defaulting to 0
    fn no_mana() -> Mana {
        Mana {
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
        }
    }
}

// define a data structure for a Card, which can have a name and a cost
#[derive(Debug)]
struct Card {
    name: String,
    mana_cost: Mana,
}

impl Card {
    // create a new Card instance
    fn new(name: String, mana_cost: Mana) -> Card {
        Card { name, mana_cost }
    }
}

fn main() {
    // create a card with a name and a mana cost
    let card: Card = Card::new(
        String::from("Lightning Bolt"),
        Mana::new(0, 0, 0, 1, 0, 0),
    );

    // print the card name and mana cost
    println!("Card Name: {}", card.name);
    println!("{:?}", card.mana_cost);

    // create a card with no mana cost
    let no_mana_card: Card = Card::new(
        String::from("No Mana Card"), 
        Mana::no_mana(),
    );

    // print the card name and mana cost
    println!("Card Name: {}", no_mana_card.name);
    println!("{:?}", no_mana_card.mana_cost);
}
