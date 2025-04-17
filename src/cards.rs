use crate::mana::Mana;

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
