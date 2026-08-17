#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

fn parse_str(s: &str, validator: impl Fn(char) -> bool) -> Result<String, usize> {
    if let Some((index, _)) = s.char_indices().find(|(_, c)| !validator(*c)) {
        return Err(index);
    }
    Ok(s.to_string())
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let d = parse_str(dna, |c| c == 'A' || c == 'C' || c == 'G' || c == 'T')?;
        Ok(Dna { dna: d })
    }

    pub fn into_rna(mut self) -> Rna {
        unsafe {
            let bytes= self.dna.as_mut_vec();
            for c in bytes {
                match c {
                    b'A' => *c = b'U',
                    b'T' => *c = b'A',
                    b'C' => *c = b'G',
                    b'G' => *c = b'C',
                    _ => (),
                }
            }
        }
        Rna {
            rna: self.dna,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let r = parse_str(rna, |c| c == 'A' || c == 'C' || c == 'G' || c == 'U')?;
        Ok(Rna { rna: r })
    }
}
