use core::hash::Hash;
use std::collections::HashSet;

pub fn union<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
	let left: HashSet<T> = left.iter().cloned().collect();
	let right: HashSet<T> = right.iter().cloned().collect();

	let mut out = left.union(&right).cloned().collect::<Vec<_>>();
	out.sort();
	out
}

pub fn intersection<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
	let left: HashSet<T> = left.iter().cloned().collect();
	let right: HashSet<T> = right.iter().cloned().collect();

	let mut out = left.intersection(&right).cloned().collect::<Vec<_>>();
	out.sort();
	out
}

pub fn difference<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
	let left: HashSet<T> = left.iter().cloned().collect();
	let right: HashSet<T> = right.iter().cloned().collect();

	let mut out = left.difference(&right).cloned().collect::<Vec<_>>();
	out.sort();
	out
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_union() {
		let a = vec![1, 2, 3, 4, 5];
		let b = vec![2, 8, 5, 10];

		let res = union(&a, &b);

		assert_eq!(res, vec![1, 2, 3, 4, 5, 8, 10]);
	}

	#[test]
	fn test_intersection() {
		let a = vec![1, 2, 3, 4, 5];
		let b = vec![2, 8, 5, 10];

		let res = intersection(&a, &b);

		assert_eq!(res, vec![2, 5]);
	}

	#[test]
	fn test_diff() {
		let a = vec![1, 2, 3, 4, 5];
		let b = vec![2, 8, 5, 10];

		let res_1 = difference(&a, &b);
		assert_eq!(res_1, vec![1, 3, 4]);

		let res_2 = difference(&b, &a);
		assert_eq!(res_2, vec![8, 10]);
	}

	#[test]
	fn test_complement() {
		let n: Vec<usize> = (1..=10).collect();
		let a = vec![1, 2, 3, 4, 5];
		let b = vec![2, 8, 5, 10];

		let res_1 = difference(&n, &a);
		assert_eq!(res_1, vec![6, 7, 8, 9, 10]);

		let res_2 = difference(&n, &b);
		assert_eq!(res_2, vec![1, 3, 4, 6, 7, 9]);
	}
}
