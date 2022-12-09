use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'G', 'T', 'C'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) { return Err(nucleotide) };
    if let Some(_dna) = dna.chars().find(|c| !NUCLEOTIDES.contains(&c)) { return Err(_dna) };
    Ok(dna.chars().filter(|&c| c==nucleotide ).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = NUCLEOTIDES.iter().map(|n| (*n, 0)).collect();
    for c in NUCLEOTIDES.iter() {
        match count(*c, dna) {
            Ok(num) => counts.entry(*c).and_modify(|v| *v += num),
            Err(c) => return Err(c),
        };
    }
    Ok(counts)
}
