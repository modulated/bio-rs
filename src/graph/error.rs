#[derive(Debug)]
pub enum GraphError {
	NodeNotFound(String),
	EdgeNotFound((String, String)),
	PathNotFound,
}

impl std::fmt::Display for GraphError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			GraphError::NodeNotFound(n) => write!(f, "Node Not Found: {}", n),
			GraphError::EdgeNotFound(n) => write!(f, "Edge Not Found: {}->{}", n.0, n.1),
			GraphError::PathNotFound => write!(f, "Path Not Found"),
		}
	}
}

impl std::error::Error for GraphError {}
