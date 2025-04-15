// definition of structs, enums, implementations

pub mod cards {
    use super::mana::Mana;
    
    // Card is an enum that has a known number of variants (card types)
    #[derive(Debug)]
    pub enum Card {
        Land(TypeLand),
        Creature(TypeCreature),
    }

    #[derive(Debug)]
    pub struct TypeLand {
        name: String,
        mana_cost: Mana,
    }

    impl TypeLand {
        // construct an instantiation of TypeLand with passed values
        pub fn new(name: String) -> TypeLand {
            TypeLand { name, mana_cost: Mana::no_mana() }
        }
    }

    // TypeCreature contains information relevant to a creature type, namely
    // name, mana cost, power, toughness. 
    #[derive(Debug)]
    pub struct TypeCreature {
        name: String,
        mana_cost: Mana,
        power: u32,
        toughness: u32,
    }

    impl TypeCreature {
        // construct an instantiation of TypeCreature with passed values
        pub fn new(name: String, mana_cost: Mana, power: u32, toughness: u32) -> TypeCreature {
            TypeCreature {
                name,
                mana_cost,
                power,
                toughness,
            }
        }
    }
}

pub mod mana {
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
}
