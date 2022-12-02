const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna{
    strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(inv_char) = dna.chars().position(|c| !DNA_NUCLEOTIDES.contains(&c)) {
            Err(inv_char)
        } else {
           Ok(Dna{ strand: dna.to_string() })
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut new_rna = String::new();
        for nuc in self.strand.chars() {
            new_rna = format!("{}{}", new_rna, RNA_NUCLEOTIDES[DNA_NUCLEOTIDES.iter().position(|x| *x==nuc).unwrap_or_default()]);
        }
        Rna{strand: new_rna}
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(inv_char) = rna.chars().position(|c| !RNA_NUCLEOTIDES.contains(&c)) {
            Err(inv_char)
        } else {
           Ok(Rna{ strand: rna.to_string() })
        }
    }
}
