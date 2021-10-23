use std::convert::TryInto;

pub fn distance<I: PartialEq, T: Clone + IntoIterator<Item = I>>(a: &T, b: &T) -> usize {
	let mut count = 0;
	for (i, j) in a.clone().into_iter().zip(b.clone().into_iter()) {
		if i != j {
			count += 1;
		}
	}

	count
}

pub fn p_distance<I: PartialEq, T: Clone + IntoIterator<Item = I>>(a: &T, b: &T) -> f64 {
	let count_int: i32 = a.clone().into_iter().count().try_into().unwrap();
	let count: f64 = count_int.try_into().unwrap();
	let dist_int: i32 = distance(a, b).try_into().unwrap();
	let int: f64 = dist_int.try_into().unwrap();
	int / count
}

pub fn matrix<I: PartialEq, T: Clone + IntoIterator<Item = I>>(slice: &[T]) -> Vec<Vec<f64>> {
	let mut out: Vec<Vec<f64>> = vec![vec![0.0; slice.len()]; slice.len()];

	for i in 0..(slice.len() - 1) {
		for j in i..slice.len() {
			let r = p_distance(&slice[i], &slice[j]);
			out[i][j] = r;
			out[j][i] = r;
		}
	}

	out
}

#[cfg(test)]
mod test {
	use super::{distance, matrix, p_distance};
	use crate::Seq;

	#[test]
	fn hamming() {
		let a = Seq::new("GAGCCTACTAACGGGAT");
		let b = Seq::new("CATCGTAATGACGGCCT");

		assert_eq!(distance(&a.iter(), &b.iter()), 7);
	}

	#[test]
	fn p_dist() {
		let a = Seq::new("TTTCCATTTA");
		let b = Seq::new("GATTCATTTC");
		let fmt = format!("{:.5}", p_distance(&a.iter(), &b.iter()));
		assert_eq!(fmt, "0.40000");
	}

	#[test]
	fn dist_matrix() {
		let input = vec![
			Seq::new("TTTCCATTTA"),
			Seq::new("GATTCATTTC"),
			Seq::new("TTTCCATTTT"),
			Seq::new("GTTCCATTTA"),
		];

		let res = vec![
			vec![0.00000, 0.40000, 0.10000, 0.10000],
			vec![0.40000, 0.00000, 0.40000, 0.30000],
			vec![0.10000, 0.40000, 0.00000, 0.20000],
			vec![0.10000, 0.30000, 0.20000, 0.00000],
		];

		assert_eq!(matrix(&input), res);
	}
}
