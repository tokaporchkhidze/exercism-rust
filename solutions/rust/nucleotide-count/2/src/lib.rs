use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    dna.chars().try_fold(0, |acc, b| match b {
        b if VALID_NUCLEOTIDES.contains(&b) => {
            Ok(acc + (b == nucleotide) as usize)
        }
        _ => Err(b),
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    dna.chars().try_for_each(|c| {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
            Ok(())
        } else {
            Err(c)
        }
    })?;
    Ok(counts)
}
