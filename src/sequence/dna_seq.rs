use crate::*;

#[derive(Debug, PartialEq)]
pub struct DNASequence {
    pub seq: Vec<DNA>
}

impl DNASequence {
    pub fn new(str: &str) -> Self {
        use std::convert::TryFrom;
        let mut vec: Vec<DNA> = Vec::new();

        for c in str.chars() {
            let dna = DNA::try_from(&c);
            if let Ok(i) = dna {
                vec.push(i);
            }            
        }

        DNASequence {
            seq: vec
        }
    }

    pub fn counts(&self) -> (u32, u32, u32, u32) {
        let mut out = (0, 0, 0, 0);

        for n in &self.seq {
            match n {
                DNA::A => out.0 += 1,
                DNA::C => out.1 += 1,
                DNA::G => out.2 += 1,
                DNA::T => out.3 += 1,
            }
        }

        out
    }

    pub fn transcribe(&self) -> RNASequence {
        let mut seq: Vec<RNA> = vec![];

        for c in &self.seq {
            seq.push(RNA::from(*c))
        }

        RNASequence {
            seq
        }
    }

    pub fn reverse_complement(&self) -> DNASequence {
        DNASequence {
            seq: self.seq.iter().rev().map(|c|c.complement()).collect()
        }
    }

    pub fn gc_content(&self) -> f64 {
        use std::convert::TryFrom;

        let mut count = 0;
        for n in &self.seq {
            match n {
                DNA::G | DNA::C => count += 1,
                _ => continue
            }
        }

        f64::from(100 * count)/f64::from(i32::try_from(self.seq.len()).unwrap())
    }

    pub fn suffix_overlap(&self, seq: &Self, len: usize) -> bool {
        
        if self.seq.len() < len {
            return false;
        }

        let tail = &self.seq[self.seq.len() - len..];

        for (i, e) in tail.iter().enumerate().take(len) {
            if e != &seq.seq[i] {
                return false;
            }
        }

        true
    }

    pub fn prefix_overlap(&self, seq: &Self, len: usize) -> bool {
        
        if seq.seq.len() < len {
            return false;
        }

        let tail = &seq.seq[seq.seq.len() - len..];

        for (i, e) in tail.iter().enumerate().take(len) {
            if e != &self.seq[i] {
                return false;
            }
        }

        true
    }

    pub fn substring(&self, pattern: &DNASequence) -> Vec<usize> {
        substring(&pattern.seq, &self.seq)
    }

    pub fn translate(&self) -> Protein {
        let mut vec = vec![];

        for n in self.seq.chunks(3) {
            if n.len() != 3 { continue; }
            let c: AminoAcid = (&Codon::new(n[0], n[1], n[2])).into();
            vec.push(c);
        }

        Protein {
            seq: vec
        }
    }
}