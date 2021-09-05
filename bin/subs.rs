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


    let a: Vec<&str> = input.split('\n').collect();
    let res = DNASequence::new(a[0]).substring(&DNASequence::new(a[1])).iter().map(|x|x.to_string()).collect::<Vec<String>>().join(" ");

    println!("{}", res);

    Ok(())
}