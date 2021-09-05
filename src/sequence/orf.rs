use crate::{DNASequence, Protein};

pub struct ORF {
    pub data: Vec<Protein>
}

impl From<DNASequence> for ORF {
    fn from(seq: DNASequence) -> Self {
        let mut out = ORF {
            data: vec![]
        };

        let revc = seq.reverse_complement();
        for i in 0..3 {
            let shifted = &seq.seq[i..];
            let mut prot = DNASequence::new(&shifted.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("")).translate();
            let cleaved = prot.cleave_start_stop();
            if let Some(res) = cleaved {
                out.data.push(res);
            }

            let shifted_revc = &revc.seq[i..];
            let mut prot_revc = DNASequence::new(&shifted_revc.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("")).translate();
            let cleaved_revc = prot_revc.cleave_start_stop();
            if let Some(res_revc) = cleaved_revc {
                out.data.push(res_revc);
            }
        }

        out
    }
}


#[cfg(test)]
mod test {
    use crate::{ORF, DNASequence};
    #[test]
    fn test_orf() {
        let input = "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG";
        let mut output = vec![
            "MLLGSFRLIPKETLIQVAGSSPCNLS",
            "M",
            "MGMTPRLGLESLLE",
            "MTPRLGLESLLE"
        ];
        output.sort();

        let mut res = ORF::from(DNASequence::new(input)).data.iter().map(|x|x.to_string()).collect::<Vec<String>>();
        res.sort();

        assert_eq!(res, output);
    }
}