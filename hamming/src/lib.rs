/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
const NUCLEOTIDES: &str = "AGTC";

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.chars().count() != s2.chars().count() {
        // Can't compare strands of different lengths
        None
    } else if s1.find(|x| !NUCLEOTIDES.contains(x)).is_some() ||
                s2.find(|x| !NUCLEOTIDES.contains(x)).is_some() {
        // If either strand has invalid nucleotides
        None
    } else {
        // zip allows one iterator to iterate over 2 separate character iterators
        // pass tuple of (s1 char, s2 char) into fold to count differences
        Some(s1
            .chars()
            .zip(s2.chars())
            .fold(0, |sum, (c1, c2)| {if c1 != c2 {sum + 1} else {sum}})
        )
    }
}
