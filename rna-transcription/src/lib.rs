const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>{
    strand: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna<'a>{
    strand: &'a str,
}

impl Dna<'_> {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(inv_char) = dna.chars().position(|c| !DNA_NUCLEOTIDES.contains(&c)) {
            Err(inv_char)
        } else {
           Ok(Dna{ strand: dna })
        }
    }

    pub fn into_rna(self) -> Rna<'static> {
        // let mut new_rna: String = String::new();
        // for nuc in self.strand.chars() {
        //     new_rna.push_str(RNA_NUCLEOTIDES[DNA_NUCLEOTIDES.iter().position(|x| *x==nuc).unwrap_or_default()].to_string().as_str());
        // }
        // Rna{strand: new_rna.clone().as_str()}
    }
}

impl Rna<'_> {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(inv_char) = rna.chars().position(|c| !RNA_NUCLEOTIDES.contains(&c)) {
            Err(inv_char)
        } else {
           Ok(Rna{ strand: rna })
        }
    }
}
