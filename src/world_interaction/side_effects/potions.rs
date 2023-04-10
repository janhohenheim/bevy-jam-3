use crate::world_interaction::side_effects::SideEffect;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Reflect, FromReflect)]
pub(crate) struct Potion {
    pub(crate) name: String,
    pub(crate) positive_side_effect: SideEffect,
    pub(crate) negative_side_effect: SideEffect,
}

pub(crate) const POTION_COUNT: usize = 3;

pub(crate) fn generate_potions() -> [Potion; POTION_COUNT] {
    let adjectives = adjectives();
    let drinks = drinks();
    let ofs = ofs();
    let side_effects: Vec<_> = SideEffect::iter().collect();
    let mut rng = thread_rng();

    let adjectives = sample(&mut rng, &adjectives, POTION_COUNT);
    let drinks = sample(&mut rng, &drinks, POTION_COUNT);
    let ofs = sample(&mut rng, &ofs, POTION_COUNT);
    let side_effects = sample(&mut rng, &side_effects, POTION_COUNT * 2);

    let generate_potion = |index: usize| {
        let adjective = adjectives[index];
        let drink = drinks[index];
        let of = ofs[index];
        let name = format!("{} {} of {}", adjective, drink, of);
        let positive_side_effect = *side_effects[index];
        let negative_side_effect = *side_effects[side_effects.len() - index - 1];
        Potion {
            name,
            positive_side_effect,
            negative_side_effect,
        }
    };

    [generate_potion(0), generate_potion(1), generate_potion(2)]
}

fn sample<'a, 'b, T>(rng: &'b mut impl Rng, slice: &'a [T], amount: usize) -> Vec<&'a T> {
    rng.sample_iter(Uniform::new(0, slice.len()))
        .take(amount)
        .map(|i| &slice[i])
        .collect()
}

fn adjectives() -> Vec<&'static str> {
    vec![
        "Aged",
        "Ancient",
        "Cool",
        "Sparkling",
        "Fizzy",
        "Frothy",
        "Fruity",
        "Glowing",
        "Gooey",
        "Gross",
        "Weird",
        "Hot",
        "Icy",
        "Mysterious",
        "Chilled",
        "Muddy",
        "Murky",
        "Disgusting",
        "Nasty",
        "Tasty",
        "Sweet",
        "Sour",
        "Salty",
        "Bitter",
        "Spicy",
        "Savory",
        "Sweaty",
        "Slimy",
    ]
}

fn drinks() -> Vec<&'static str> {
    vec![
        "Ale",
        "Beer",
        "Brew",
        "Cider",
        "Juice",
        "Lager",
        "Liquor",
        "Potion",
        "Elixir",
        "Soda",
        "Slurpy",
        "Smoothie",
        "Energy Drink",
        "Milkshake",
        "Water",
        "Wine",
        "Tea",
        "Coffee",
        "Cocoa",
    ]
}

fn ofs() -> Vec<&'static str> {
    vec![
        "the Bodhisattva ",
        "the Buddha",
        "Caesar",
        "the Kind",
        "the Wizard",
        "Jesus",
        "the Pope",
        "the Warrior",
        "the Monk",
        "the Sage",
        "the Dark One",
        "Enlightenment",
        "the Hanged Man",
        "the Fool",
        "the Magician",
        "the High Priestess",
        "the Empress",
        "the Emperor",
        "the Hierophant",
        "the Lovers",
        "the Chariot",
        "Justice",
        "the Hermit",
        "the Astral Traveler",
        "the Outlaw",
        "the Outcast",
        "the Devil",
        "the Tower",
        "Hohenheim",
        "Bevy",
    ]
}
