pub struct Allergies { score: usize }

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as usize }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as usize != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergies: Vec<Allergen> = vec![Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats];
        let mut al: Vec<Allergen> = vec![]; 
        for allergen in allergies.iter() {
            if self.is_allergic_to(&allergen) { al.push(*allergen)};
        }
        al
    }
}
