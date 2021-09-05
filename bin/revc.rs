use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("REVC Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    let filename = args().nth(1).ok_or("File not found.")?;
    let input = std::fs::read_to_string(filename)?;




    let seq = DNASequence::new(&input).reverse_complement();
    

    println!("{}", seq);
    
    Ok(())
}
