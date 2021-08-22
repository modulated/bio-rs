use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FIB Problem");
    // if args().len() < 2 {
    //     println!("Please supply file as argument.");
    //     return Ok(());
    // }

    // let filename = args().skip(1).next().ok_or("File not found.")?;
    // let input = std::fs::read_to_string(filename)?;
    // let v: Vec<u32> = input.split(' ').map(|s| s.parse().unwrap()).collect();
    
    let mut memo = std::collections::HashMap::new();
    let res = util::fibonacci_rabbits(36, 3, &mut memo);
    println!("{}", res);
    
    Ok(())
}

#[cfg(test)]
mod test {
    use bio::*;
    #[test]
    fn sample() {
        let input = "5 3";
        let output = "19";
        let v: Vec<u64> = input.split(' ').map(|s| s.parse().unwrap()).collect();

        let mut memo = std::collections::HashMap::new();
        let res = util::fibonacci_rabbits(v[0], v[1], &mut memo);
                
        assert_eq!(output, res.to_string());
    }
}