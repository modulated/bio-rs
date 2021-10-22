use std::collections::HashSet;
use std::iter::FromIterator;
use core::hash::Hash;

pub fn union<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    let left: HashSet<T> = HashSet::from_iter(left.iter().cloned());
    let right = HashSet::from_iter(right.iter().cloned());

    let mut out = right.union(&left).cloned().collect::<Vec<_>>();
    out.sort();
    out
}

pub fn intersection<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    let left: HashSet<T> = HashSet::from_iter(left.iter().cloned());
    let right = HashSet::from_iter(right.iter().cloned());

    let mut out = right.intersection(&left).cloned().collect::<Vec<_>>();
    out.sort();
    out
}

pub fn difference<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    todo!();
}

pub fn complement<T: Clone + Eq + Hash + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn union1() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![2, 8, 5, 10];

        let res = union(&a, &b);

        assert_eq!(res, vec![1, 2, 3, 4, 5, 8, 10]);
    }
}