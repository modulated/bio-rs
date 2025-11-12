use super::Error;
use std::{
	borrow::Borrow,
	collections::{HashMap, HashSet, VecDeque},
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Unweighted<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> {
	data: HashMap<T, Vec<T>>,
}

impl<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> Default for Unweighted<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> Unweighted<T> {
	#[must_use]
	pub fn new() -> Self {
		Self {
			data: HashMap::new(),
		}
	}

	pub fn add(&mut self, node: T) {
		self.data.insert(node, Vec::new());
	}

	#[must_use]
	pub fn num_nodes(&self) -> usize {
		self.data.len()
	}

	#[must_use]
	pub fn num_edges(&self) -> usize {
		let mut n = 0;
		for i in self.data.values() {
			n += i.len();
		}

		n / 2
	}

	#[must_use]
	pub fn nodes(&self) -> Vec<T> {
		self.data.keys().cloned().collect()
	}

	pub fn neighbours(&self, node: &T) -> Result<Vec<T>> {
		let r = self
			.data
			.get(node)
			.ok_or_else(|| Error::NodeNotFound(node.to_string()))?;
		Ok(r.clone())
	}

	pub fn degree(&self, node: &T) -> Result<usize> {
		let r = self
			.data
			.get(node)
			.ok_or_else(|| Error::NodeNotFound(node.to_string()))?;
		Ok(r.len())
	}

	#[must_use]
	pub fn iter(&self) -> std::collections::hash_map::Iter<T, Vec<T>> {
		self.data.iter()
	}

	pub fn dedup(&mut self) {
		for k in self.data.clone().keys() {
			let e = self.data.entry(k.clone()).or_default();
			e.sort_unstable();
			e.dedup();
		}
	}

	pub fn prune(&mut self, thresh: usize) {
		self.data.retain(|_, v| v.len() > thresh);
	}

	pub fn link<Q: Borrow<T>>(&mut self, node_1: Q, node_2: Q) -> Result<()> {
		if !self.data.contains_key(node_1.borrow()) {
			return Err(Error::NodeNotFound(node_1.borrow().to_string()));
		}
		if !self.data.contains_key(node_2.borrow()) {
			return Err(Error::NodeNotFound(node_2.borrow().to_string()));
		}

		let mut e = self.data.entry(node_1.borrow().clone()).or_default();
		e.push(node_2.borrow().clone());

		e = self.data.entry(node_2.borrow().clone()).or_default();
		e.push(node_1.borrow().clone());

		Ok(())
	}

	pub fn unlink<Q: Borrow<T>>(&mut self, node_1: Q, node_2: Q) -> Result<()> {
		if !self.data.contains_key(node_1.borrow()) {
			return Err(Error::NodeNotFound(node_1.borrow().to_string()));
		}
		if !self.data.contains_key(node_2.borrow()) {
			return Err(Error::NodeNotFound(node_2.borrow().to_string()));
		}

		let edges_1 = self.data.entry(node_1.borrow().clone()).or_default();
		if !edges_1.contains(node_2.borrow()) {
			return Err(Error::EdgeNotFound((
				node_1.borrow().to_string(),
				node_2.borrow().to_string(),
			)));
		}
		edges_1.retain(|x| x != node_2.borrow());

		let edges_2 = self.data.entry(node_2.borrow().clone()).or_default();
		if !edges_2.contains(node_1.borrow()) {
			return Err(Error::EdgeNotFound((
				node_2.borrow().to_string(),
				node_1.borrow().to_string(),
			)));
		}
		edges_2.retain(|x| x != node_1.borrow());

		Ok(())
	}

	#[must_use]
	pub fn is_tree(&self) -> bool {
		self.connected() && (self.num_edges() == self.num_nodes() - 1)
	}

	#[must_use]
	pub fn is_ubt(&self) -> bool {
		self.connected()
			&& self
				.nodes()
				.iter()
				.all(|node| (self.degree(node).unwrap() == 3 || self.degree(node).unwrap() == 1))
	}

	#[must_use]
	pub fn edges_required_for_tree(&self) -> usize {
		self.num_nodes() - 1 - self.num_edges()
	}

	#[must_use]
	pub fn ubt_internal_count(&self) -> usize {
		assert!(self.is_ubt());
		self.ubt_leaf_count() - 2
	}

	#[must_use]
	pub fn ubt_leaf_count(&self) -> usize {
		self.iter().fold(0, |accum, x| match x.1.len() {
			1 => accum + 1,
			_ => accum,
		})
	}

	#[must_use]
	pub fn connected(&self) -> bool {
		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		let root = self.data.keys().next().unwrap();
		visited.insert(root.clone());
		queue.push_back(root.clone());

		while !queue.is_empty() {
			let n = queue.pop_front().unwrap();
			visited.insert(n.clone());
			let neighbours = self.neighbours(&n).unwrap();
			for node in neighbours {
				if !visited.contains(&node) {
					queue.push_back(node);
				}
			}
		}

		visited.len() == self.num_nodes()
	}

	pub fn bfs_path(&self, start: T, end: &T) -> Result<Vec<T>> {
		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();

		queue.push_back(vec![start]);

		while !queue.is_empty() {
			let path = queue.pop_front().unwrap();

			// return if path ends in destination
			if path.last().unwrap() == end {
				return Ok(path);
			}

			for node in self.neighbours(path.last().unwrap())? {
				if !visited.contains(&node) {
					visited.insert(node.clone());
					let mut new_path = path.clone();
					new_path.push(node);
					queue.push_back(new_path);
				}
			}
		}

		Err(Error::PathNotFound)
	}
}

impl<'a, T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> IntoIterator
	for &'a Unweighted<T>
{
	type Item = (&'a T, &'a std::vec::Vec<T>);

	type IntoIter = std::collections::hash_map::Iter<'a, T, std::vec::Vec<T>>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(test)]
mod tests {
	use super::Unweighted;
	#[test]
	fn unweighted_size() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		assert_eq!(g.num_nodes(), 2);
	}

	#[test]
	fn unweighted_link() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(3, 2).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.num_edges(), 3);
	}

	#[test]
	fn unweighted_unlink() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(3, 2).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.num_edges(), 3);

		g.unlink(1, 2).unwrap();
		assert_eq!(g.num_edges(), 2);
		assert!(g.unlink(1, 2).is_err());
	}

	#[test]
	#[should_panic(expected = "NodeNotFound")]
	fn unweighted_link_error() {
		let mut g = Unweighted::new();
		g.add(2);
		g.link(1, 2).unwrap();
	}

	#[test]
	fn unweighted_neighbours() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(2, 3).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.neighbours(&2).unwrap(), vec![1, 3]);
		assert_eq!(g.degree(&2).unwrap(), 2);
	}

	#[test]
	fn unweighted_bfs_path() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(3);
		g.add(2);
		g.add(4);
		g.add(5);
		g.add(6);
		g.link(1, 2).unwrap();
		g.link(2, 3).unwrap();
		g.link(2, 6).unwrap();
		g.link(3, 5).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.bfs_path(1, &4).unwrap(), vec![1, 2, 3, 4]);
	}

	#[test]
	#[should_panic(expected = "PathNotFound")]
	fn unweighted_bfs_path_fail() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(3);
		g.add(2);
		g.add(4);
		g.add(5);
		g.add(6);
		g.link(1, 2).unwrap();
		g.link(2, 3).unwrap();
		g.link(3, 5).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.bfs_path(1, &6).unwrap(), vec![1, 2, 3, 4]);
	}

	#[test]
	fn unweighted_connected() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(3, 2).unwrap();
		g.link(3, 4).unwrap();

		assert!(g.connected());
	}

	#[test]
	#[should_panic(expected = "assertion failed")]
	fn unweighted_connected_fail() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(3, 2).unwrap();
		g.link(3, 4).unwrap();
		g.add(5);

		assert!(g.connected());
	}

	#[test]
	fn unweighted_is_tree() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.add(5);
		g.add(6);
		g.add(7);
		g.link(1, 2).unwrap();
		g.link(1, 3).unwrap();
		assert!(!g.is_tree());
		g.link(2, 4).unwrap();
		g.link(2, 5).unwrap();
		g.link(3, 6).unwrap();
		g.link(3, 7).unwrap();

		assert!(g.is_tree());

		// Iterate every node and make sure unlinking breaks tree property
		for x in 1..=g.num_nodes() {
			for y in x..g.num_nodes() {
				if x == y {
					continue;
				}
				let mut n = g.clone();
				match n.unlink(x, y) {
					Err(_) => {}
					Ok(()) => assert!(!n.is_tree()),
				}
			}
		}
	}

	#[test]
	fn unweighted_min_tree() {
		let mut g = Unweighted::new();

		for x in 1..=10 {
			g.add(x);
		}

		g.link(1, 2).unwrap();
		g.link(2, 8).unwrap();
		g.link(4, 10).unwrap();
		g.link(5, 9).unwrap();
		g.link(6, 10).unwrap();
		g.link(7, 9).unwrap();

		assert_eq!(g.edges_required_for_tree(), 3);
	}

	#[test]
	fn unweighted_ubt() {
		let mut g = Unweighted::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.add(5);
		g.add(6);
		g.link(1, 2).unwrap();
		g.link(2, 3).unwrap();
		g.link(2, 4).unwrap();
		g.link(1, 5).unwrap();
		g.link(1, 6).unwrap();

		assert!(g.is_ubt());
		assert_eq!(g.ubt_leaf_count(), 4);
		assert_eq!(g.ubt_internal_count(), 2);

		g.link(1, 4).unwrap();
		assert!(!g.is_ubt());
	}
}
