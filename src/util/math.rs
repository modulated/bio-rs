use num::{BigUint, ToPrimitive};
use std::convert::TryInto;

#[must_use]
pub fn ncr(n: u64, r: u64) -> BigUint {
	let r = r.min(n - r);
	if r == 0 {
		return BigUint::from(1_u64);
	}
	let numerator: BigUint = ((n - r + 1)..=n).product();
	let denominator: BigUint = (1..=r).product();
	numerator / denominator
}

#[must_use]
pub fn modulo_factorial(num: u64, m: u64) -> u64 {
	(1..=num).fold(1, |acc, v| (acc * v) % m)
}

#[must_use]
pub fn factorial(num: u64) -> BigUint {
	let mut accum = BigUint::from(1_u32);
	for i in 1..=num {
		accum *= BigUint::from(i);
	}
	accum
}

#[must_use]
pub fn poisson_pdf(x: u32, lambda: f64) -> f64 {
	(-lambda).exp() * lambda.powi(x.try_into().unwrap()) / factorial(x.into()).to_f64().unwrap()
}

#[must_use]
pub fn poisson_cdf(x: u32, lambda: f64) -> f64 {
	(0..=x).into_iter().map(|x| poisson_pdf(x, lambda)).sum()
}

#[cfg(test)]
mod test {
	use super::{factorial, modulo_factorial, poisson_cdf, poisson_pdf, BigUint};
	#[test]
	fn test_factorial() {
		assert_eq!(factorial(6), BigUint::from(720_u32));
	}

	#[test]
	fn test_mod_factorial() {
		assert_eq!(modulo_factorial(6, 1000), 720);
		assert_eq!(modulo_factorial(6, 700), 20);
	}

	#[test]
	fn test_poisson_pdf() {
		let x = 1;
		let lambda = 0.5;
		let res = format!("{:.14}", poisson_pdf(x, lambda));

		assert_eq!(res, "0.30326532985632");
	}

	#[test]
	fn test_poisson_cdf() {
		let x = 4;
		let lambda = 0.01;
		let res = format!("{:.14}", poisson_cdf(x, lambda));

		assert_eq!(res, "0.99999999999917");
	}
}
