use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNA Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    let filename = args().nth(1).ok_or("A")?;
    let input = std::fs::read_to_string(filename)?;




    let seq = DNASequence::new(&input).transcribe();
    

    println!("{}", seq);
    
    Ok(())
}