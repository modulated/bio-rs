use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PROT Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    
    let filename = args().skip(1).next().ok_or("File not found.")?;
    let input = std::fs::read_to_string(filename)?;
    
    let rna = RNASequence::new(&input);
    let prot = rna.transcribe().translate();

    println!("{}", prot);

    Ok(())
}

#[cfg(test)]
mod test {
    use bio::*;    
    #[test]
    fn sample_prot() {
        let input = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        let output = "MAMAPRTEINSTRING";
        
            
                
        assert_eq!(output, RNASequence::new(input).transcribe().translate().to_string());
    }
}