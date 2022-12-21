use std::collections::HashMap;
#[derive(Debug)]
pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars().collect::<Vec<char>>()
            .chunks(3)
            .map(|ch| self.name_for(ch.iter().collect::<String>().as_ref()))
            .take_while(|&name| name != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let codons = pairs.into_iter().collect();
    CodonsInfo { codons }
}
