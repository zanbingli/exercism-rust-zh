pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq)]
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
        Allergies {
            score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return match allergen {
            Allergen::Eggs => self.score & 01 > 0,
            Allergen::Peanuts => self.score & 02 > 0,
            Allergen::Shellfish => self.score & 04 > 0,
            Allergen::Strawberries => self.score & 08 > 0,
            Allergen::Tomatoes => self.score & 16 > 0,
            Allergen::Chocolate => self.score & 32 > 0,
            Allergen::Pollen => self.score & 64 > 0,
            Allergen::Cats => self.score & 128 > 0,
        };
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        vec![Allergen::Eggs,
             Allergen::Peanuts,
             Allergen::Shellfish,
             Allergen::Strawberries,
             Allergen::Tomatoes,
             Allergen::Chocolate,
             Allergen::Pollen, Allergen::Cats].into_iter().filter(|x| self.is_allergic_to(x))
            .collect()
    }
}
