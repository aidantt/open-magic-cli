// Mana tracks any combination of mana types, and may represent any quantity
// of Mana (costs, available mana, etc.)
#[derive(Debug)]
pub struct Mana {
    colorless: u32,
    white: u32,
    blue: u32,
    black: u32,
    red: u32,
    green: u32,
}

impl Mana {
    // construct an instantiation of Mana with passed values
    pub fn new(
        colorless: u32,
        white: u32,
        blue: u32,
        black: u32,
        red: u32,
        green: u32,
    ) -> Mana {
        Mana {
            colorless,
            white,
            blue,
            black,
            red,
            green,
        }
    }

    // construct an instantiation of Mana with no mana
    // this is useful for lands, which nominally have no mana cost, but might
    // still need the capacity to track mana
    pub fn no_mana() -> Mana {
        Mana {
            colorless: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
        }
    }
}
