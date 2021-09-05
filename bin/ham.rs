use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HAMM Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }
    let filename = args().nth(1).ok_or("File not found.")?;
    let input = std::fs::read_to_string(filename)?;
    
    let parsed: Vec<String> = input.split('\n').map(|x| x.split_whitespace().collect()).collect();
                
    let a = DNASequence::new(&parsed[0]);
    let b = DNASequence::new(&parsed[1]);

    let h = hamming_distance(&a.seq, &b.seq);

    println!("{}", h);

    Ok(())
}
