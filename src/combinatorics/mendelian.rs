use num::ToPrimitive;
use std::convert::TryInto;

/// Returns probability of two randomly selected organisms producing an individual possessing a dominant allele.
#[must_use]
pub fn prob_inheritance_dominant(
	homozygous_dominant: u32,
	heterozygous_dominant: u32,
	homozygous_recesive: u32,
) -> f64 {
	let m = f64::from(heterozygous_dominant);
	let n = f64::from(homozygous_recesive);
	let t = f64::from(homozygous_dominant + heterozygous_dominant + homozygous_recesive);

	(1.0 / t / (t - 1.0)).mul_add(-(n.mul_add(n - 1.0, n * m) + m * (m - 1.0) / 4.0), 1.0)
}

/// Returns the probability that at least `n` organisms will be `AaBb` after `k` generations if they only mate with `AaBb` individuals
#[must_use]
pub fn prob_heterozygous_child(k: u32, n: u32) -> f64 {
	use crate::util::math::ncr;
	let total = 2_u64.pow(k);
	(u64::from(n)..=total)
		.map(|i| {
			ncr(total, i).to_f64().unwrap()
				* 0.25_f64.powi(i.try_into().unwrap())
				* 0.75_f64.powi((total - i).try_into().unwrap())
		})
		.sum::<f64>()
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
