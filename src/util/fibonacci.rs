#[allow(clippy::implicit_hasher)]
pub fn rabbits(
	months: u64,
	litter_size: u64,
	memo: &mut std::collections::HashMap<(u64, u64), u64>,
) -> u64 {
	let args = (months, litter_size);

	if memo.contains_key(&args) {
		return memo[&args];
	}

	match months {
		1 | 2 => 1,
		_ => {
			rabbits(months - 1, litter_size, memo)
				+ litter_size * rabbits(months - 2, litter_size, memo)
		}
	}
}

#[allow(clippy::implicit_hasher)]
pub fn mortal_rabbits(
	months: i64,
	lifespan: i64,
	memo: &mut std::collections::HashMap<i64, i64>,
) -> i64 {
	if memo.contains_key(&months) {
		return memo[&months];
	}

	let out = match months {
		x if x < 0 => 0,
		0 => 0,
		1 => 1,
		_ => {
			if months <= lifespan {
				mortal_rabbits(months - 1, lifespan, memo)
					+ mortal_rabbits(months - 2, lifespan, memo)
			} else if months == lifespan + 1 {
				mortal_rabbits(months - 1, lifespan, memo)
					+ mortal_rabbits(months - 2, lifespan, memo)
					- 1
			} else {
				mortal_rabbits(months - 1, lifespan, memo)
					+ mortal_rabbits(months - 2, lifespan, memo)
					- mortal_rabbits(months - (lifespan + 1), lifespan, memo)
			}
		}
	};

	memo.insert(months, out);
	out
}

#[cfg(test)]
mod test {
	use super::mortal_rabbits;

	#[test]
	fn test_mortal() {
		let mut map = std::collections::HashMap::new();
		assert_eq!(mortal_rabbits(6, 3, &mut map), 4);
		println!("{:?}", map);
	}
}
