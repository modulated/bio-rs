pub fn fibonacci_rabbits(
	months: u64,
	litter_size: u64,
	memo: &mut std::collections::HashMap<(u64, u64), u64>,
) -> u64 {
	let args = (months, litter_size);

	if memo.contains_key(&args) {
		return memo[&args];
	}

	match months {
		1 => 1,
		2 => 1,
		_ => {
			fibonacci_rabbits(months - 1, litter_size, memo)
				+ litter_size * fibonacci_rabbits(months - 2, litter_size, memo)
		}
	}
}
