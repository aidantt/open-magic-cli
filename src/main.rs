// open magic cli, an attempt at magic in the terminal

use open_magic_cli::{
    cards::{Card, TypeCreature, TypeLand},
    mana::Mana,
};

fn main() {
    // instantiate a creature
    let creature = TypeCreature::new(
        String::from("Goblin"),
        Mana::new(0, 1, 0, 0, 0, 0),
        2,
        2,
    );
    // create a card from this creature description
    let card = Card::Creature(creature);
    // print the card
    println!("{:?}", card);

    // instantiate a land
    let land = TypeLand::new(String::from("Forest"));
    // create a card from this land description
    let land_card = Card::Land(land);
    // print the card
    println!("{:?}", land_card);
}
