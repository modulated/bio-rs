use bio::util::mortal_fibonacci_rabbits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("FIBD Problem");

    let args: Vec<_> = std::env::args().skip(1).take(2).collect();
    if args.len() != 2 {
        panic!("Please supply 2 integers as args");
    }
    
    let (months, lifespan) = (args[0].parse().unwrap(), args[1].parse().unwrap()); 

	let mut memo = std::collections::HashMap::new();
	let res = mortal_fibonacci_rabbits(months, lifespan, &mut memo);
	println!("{}", res);

	Ok(())
}
