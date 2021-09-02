use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FIB Problem");

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