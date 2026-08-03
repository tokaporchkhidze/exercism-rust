use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub struct Allergies(u32);

#[derive(EnumIter, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {

    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => (self.0 & 1) == 1,
            Allergen::Peanuts => (self.0 & 2) == 2,
            Allergen::Shellfish => (self.0 & 4) == 4,
            Allergen::Strawberries => (self.0 & 8) == 8,
            Allergen::Tomatoes => (self.0 & 16) == 16,
            Allergen::Chocolate => (self.0 & 32) == 32,
            Allergen::Pollen => (self.0 & 64) == 64,
            Allergen::Cats => (self.0 & 128) == 128,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = vec![];
        for allergen in Allergen::iter() {
            if self.is_allergic_to(&allergen) {
                res.push(allergen);
            }
        }
        res
    }
}
