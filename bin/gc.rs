use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GC Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    let filename = args().skip(1).next().ok_or("File not found.")?;
    let input = std::fs::read_to_string(filename)?;

    let mut map = std::collections::HashMap::new();

    let fastas = fasta::parse_string_to_vec_of_fasta(&input);
    for f in fastas {
        map.insert(f.name, f.seq.gc_content());
    }

    let max = map.iter().max_by(|a,b| (*a.1).partial_cmp(b.1).unwrap()).unwrap();
    println!("{} {}", *max.0, *max.1);
    
    Ok(())
}

#[cfg(test)]
mod test {
    use bio::*;
    #[test]
    fn sample() {
        let input = ">Rosalind_6404
        CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
        TCCCACTAATAATTCTGAGG
        >Rosalind_5959
        CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
        ATATCCATTTGTCAGCAGACACGC
        >Rosalind_0808
        CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
        TGGGAACCTGCGGGCAGTAGGTGGAAT";
        let output = 
        "Rosalind_0808 60.919540";

        let mut map = std::collections::HashMap::new();

        let fastas = fasta::parse_string_to_vec_of_fasta(&input);
        for f in fastas {
            map.insert(f.name, f.seq.gc_content());
        }
    
        let max = map.iter().max_by(|a,b| (*a.1).partial_cmp(b.1).unwrap()).unwrap();
        let res = format!("{} {}", *max.0, &max.1.to_string()[..9]);
                
        assert_eq!(output, res);
    }
}