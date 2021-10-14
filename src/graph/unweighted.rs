use super::GraphError;
use std::collections::{HashMap, HashSet, VecDeque};

type Result<T> = std::result::Result<T, GraphError>;

#[derive(Clone)]
pub struct UnweightedGraph<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> {
	data: HashMap<T, Vec<T>>,
}

impl<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> Default for UnweightedGraph<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Eq + core::hash::Hash + std::fmt::Display + Clone + Ord> UnweightedGraph<T> {
	pub fn new() -> Self {
		UnweightedGraph {
			data: HashMap::new(),
		}
	}

	pub fn add(&mut self, node: T) {
		self.data.insert(node, Vec::new());
	}

	pub fn num_nodes(&self) -> usize {
		self.data.len()
	}

	pub fn num_edges(&self) -> usize {
		let mut n = 0;
		for i in self.data.values() {
			n += i.len();
		}

		n / 2
	}

	pub fn nodes(&self) -> Vec<T> {
		self.data.keys().cloned().collect()
	}

	pub fn neighbours(&self, node: &T) -> Result<Vec<T>> {
		let r = self
			.data
			.get(node)
			.ok_or_else(|| GraphError::NodeNotFound(node.to_string()))?;
		Ok(r.clone())
	}

	pub fn degree(&self, node: &T) -> Result<usize> {
		let r = self
			.data
			.get(node)
			.ok_or_else(|| GraphError::NodeNotFound(node.to_string()))?;
		Ok(r.len())
	}

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

	pub fn link(&mut self, node_1: T, node_2: T) -> Result<()> {
		if !self.data.contains_key(&node_1) {
			return Err(GraphError::NodeNotFound(node_1.to_string()));
		}
		if !self.data.contains_key(&node_2) {
			return Err(GraphError::NodeNotFound(node_2.to_string()));
		}

		let e = self.data.entry(node_1.clone()).or_default();
		e.push(node_2.clone());

		let e = self.data.entry(node_2).or_default();
		e.push(node_1);

		Ok(())
	}

	pub fn unlink(&mut self, node_1: T, node_2: T) -> Result<()> {
		if !self.data.contains_key(&node_1) {
			return Err(GraphError::NodeNotFound(node_1.to_string()));
		}
		if !self.data.contains_key(&node_2) {
			return Err(GraphError::NodeNotFound(node_2.to_string()));
		}

		let edges_1 = self.data.entry(node_1.clone()).or_insert_with(Vec::new);
		if !edges_1.contains(&node_2) {
			return Err(GraphError::EdgeNotFound((
				node_1.to_string(),
				node_2.to_string(),
			)));
		}
		edges_1.retain(|x| x != &node_2);

		let edges_2 = self.data.entry(node_2.clone()).or_insert_with(Vec::new);
		if !edges_2.contains(&node_1) {
			return Err(GraphError::EdgeNotFound((
				node_2.to_string(),
				node_1.to_string(),
			)));
		}
		edges_2.retain(|x| x != &node_1);

		Ok(())
	}

	pub fn is_tree(&self) -> bool {
		self.connected() && (self.num_edges() == self.num_nodes() - 1)
	}

	pub fn is_ubt(&self) -> bool {
		self.connected() && self.nodes().iter().all(|node| (self.degree(node).unwrap() == 3 || self.degree(node).unwrap() == 1))
	}

	pub fn edges_required_for_tree(&self) -> usize {
		self.num_nodes() - 1 - self.num_edges()
	}

	pub fn ubt_internal_count(&self) -> usize {
		assert!(self.is_ubt());
		self.ubt_leaf_count() - 2
	}

	pub fn ubt_leaf_count(&self) -> usize {
		self.iter().fold(0,|accum, x|{
			match x.1.len() {
				1 => accum + 1,
				_ => accum
			}
		})
	}

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
					queue.push_back(node)
				}
			}
		}

		visited.len() == self.num_nodes()
	}

	pub fn bfs_path(&self, start: T, end: T) -> Result<Vec<T>> {
		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();

		queue.push_back(vec![start]);

		while !queue.is_empty() {
			let path = queue.pop_front().unwrap();

			// return if path ends in destination
			if path.last().unwrap() == &end {
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

		Err(GraphError::PathNotFound)
	}
}

#[cfg(test)]
mod tests {
	use super::UnweightedGraph;
	#[test]
	fn unweighted_size() {
		let mut g = UnweightedGraph::new();
		g.add(1);
		g.add(2);
		assert_eq!(g.num_nodes(), 2);
	}

	#[test]
	fn unweighted_link() {
		let mut g = UnweightedGraph::new();
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
		let mut g = UnweightedGraph::new();
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
		let mut g = UnweightedGraph::new();
		g.add(2);
		g.link(1, 2).unwrap();
	}

	#[test]
	fn unweighted_neighbours() {
		let mut g = UnweightedGraph::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.link(1, 2).unwrap();
		g.link(2, 3).unwrap();
		g.link(3, 4).unwrap();

		assert_eq!(g.neighbours(&2).unwrap(), vec!(1, 3));
		assert_eq!(g.degree(&2).unwrap(), 2);
	}

	#[test]
	fn unweighted_bfs_path() {
		let mut g = UnweightedGraph::new();
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

		assert_eq!(g.bfs_path(1, 4).unwrap(), vec![1, 2, 3, 4]);
	}

	#[test]
	#[should_panic(expected = "PathNotFound")]
	fn unweighted_bfs_path_fail() {
		let mut g = UnweightedGraph::new();
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

		assert_eq!(g.bfs_path(1, 6).unwrap(), vec![1, 2, 3, 4]);
	}

	#[test]
	fn unweighted_connected() {
		let mut g = UnweightedGraph::new();
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
	#[should_panic]
	fn unweighted_connected_fail() {
		let mut g = UnweightedGraph::new();
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
		let mut g = UnweightedGraph::new();
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
					Ok(_) => assert!(!n.is_tree()),
				}
			}
		}
	}

	#[test]
	fn unweighted_min_tree() {
		let mut g = UnweightedGraph::new();

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
		let mut g = UnweightedGraph::new();
		g.add(1);
		g.add(2);
		g.add(3);
		g.add(4);
		g.add(5);
		g.add(6);
		g.link(1,2).unwrap();
		g.link(2,3).unwrap();
		g.link(2,4).unwrap();
		g.link(1,5).unwrap();
		g.link(1,6).unwrap();

		assert!(g.is_ubt());
		assert_eq!(g.ubt_leaf_count(), 4);
		assert_eq!(g.ubt_internal_count(), 2);
		
		g.link(1, 4).unwrap();
		assert!(!g.is_ubt());
	}
}
