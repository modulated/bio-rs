use std::env::args;
use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GRPH Problem");
    if args().len() < 2 {
        println!("Please supply file as argument.");
        return Ok(());
    }

    
    let filename = args().skip(1).next().ok_or("File not found.")?;
    let input = std::fs::read_to_string(filename)?;
    
    let fastas = fasta::parse_string_to_vec_of_fasta(&input);
    println!("{} sequences loaded", fastas.len());

    let edges = crate::overlap_graph(fastas, 3);    

    for (k,v) in &edges {
        println!("{} {}", k, v);
    }

    println!("{} edges found", &edges.len());
    
    Ok(())
}

#[cfg(test)]
mod test {
    use bio::*;    
    #[test]
    fn sample_grph() {
        let input = 
        ">Rosalind_0498
        AAATAAA
        >Rosalind_2391
        AAATTTT
        >Rosalind_2323
        TTTTCCC
        >Rosalind_0442
        AAATCCC
        >Rosalind_5013
        GGGTGGG";
        let output: Vec<(String, String)> = vec![("Rosalind_0498", "Rosalind_2391"),
        ("Rosalind_0498", "Rosalind_0442"),
        ("Rosalind_2391", "Rosalind_2323")].into_iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();

        let fastas = fasta::parse_string_to_vec_of_fasta(input);

        let g = overlap_graph(fastas, 3);        
                
        assert_eq!(output, g);
    }
}