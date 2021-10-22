use bio::graph::Unweighted;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("TREE Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;

	let mut iter = input.trim().split('\n');
	let n: usize = iter.next().unwrap().parse()?;

	println!("Nodes: {}", n);

	let edges: Vec<Vec<usize>> = iter
		.map(|x| x.split(' ').map(|y| y.parse().unwrap()).collect::<Vec<_>>())
		.collect();

	println!("Edges: {:#?}", edges.len());

	let mut g = Unweighted::new();
	for i in 1..=n {
		g.add(i);
	}
	for e in edges {
		g.link(e[0], e[1])?;
	}

	println!("{}", g.edges_required_for_tree());

	Ok(())
}
