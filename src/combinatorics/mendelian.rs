/// Returns probability of two randomly selected organisms producing an individual possessing a dominant allele.
pub fn prob_inheritance_dominant(
	homozygous_dominant: u32,
	heterozygous_dominant: u32,
	homozygous_recesive: u32,
) -> f64 {
	let m = f64::from(heterozygous_dominant);
	let n = f64::from(homozygous_recesive);
	let t = f64::from(homozygous_dominant + heterozygous_dominant + homozygous_recesive);

	1.0 - 1.0 / t / (t - 1.0) * (n * (n - 1.0) + n * m + m * (m - 1.0) / 4.0)
}

/// Returns the probability that at least `n` organisms will be AaBb after `k` generations if they only mate with AaBb individuals
pub fn prob_heterozygous_child(k: u32, n: u32) -> f64 {
	let total = 2_u32.pow(k);
	let prob_het: f64 = 4.0 / 16.0;
	let mut prob = vec![];

	for r in n..=total {
		prob.push(
			combinatorial(total.into(), r.into())
				* prob_het.powi(r as i32)
				* ((1.0 - prob_het).powi((total - r) as i32)),
		);
	}

	prob.iter().sum()
}

fn combinatorial(n: u64, r: u64) -> f64 {
	use crate::combinatorics::permutation::factorial as f;
	f(n) as f64 / f(r) as f64 / f(n - r) as f64
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn dominant() {
		assert_eq!(
			&prob_inheritance_dominant(2, 2, 2).to_string()[..7],
			"0.78333"
		);
	}

	#[test]
	fn hetero() {
		assert_eq!(format!("{:.1$}", prob_heterozygous_child(2, 1), 3), "0.684");
	}
}
