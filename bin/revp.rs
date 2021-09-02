use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("REVP Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    let filename = args().skip(1).next().ok_or("Error: Unable to read args")?;
    let input = std::fs::read_to_string(filename)?;

    let seq = &fasta::parse_string_to_vec_of_fasta(&input)[0].seq;

    let res = reverse_palindrome(&seq.seq, 4, 12);

    let mut s = "".to_string();
    for (i,j) in res {
        use std::fmt::Write;
        write!(s, "{} {}\n", i, j)?;
    }
    println!("{}", s);

    Ok(())
}