pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                nodes: (String, String),
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Edge {
                    Edge {
                        nodes: (node1.to_string(), node2.to_string()),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub(crate) name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    self.attrs
                        .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn node(&self, key: &str) -> Option<&Node> {
            self.nodes.iter().find(|&val| val.name == key)
        }
    }
}
