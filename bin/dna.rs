use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DNA Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    let filename = args().nth(1).unwrap();
    let input = std::fs::read_to_string(filename)?;

    let seq = DNASequence::new(&input);
    let res = seq.counts();

    println!("{} {} {} {}", res.0, res.1, res.2, res.3);
    
    Ok(())
}