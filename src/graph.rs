use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Option<isize>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn get_node(&mut self, index: usize) -> &mut Node<T> {
        &mut self.nodes[index]
    }

    pub fn get_node_amount(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge_amount(&self) -> usize {
        self.edges.iter().filter(|e| e.is_some()).count()
    }

    pub fn add_node(&mut self, element: T) -> usize {
        self.nodes.push(Node {
            element: element,
            visited: false,
        });
        let extension = self.nodes.len().pow(2) - (self.nodes.len() - 1).pow(2);
        let mut vec: Vec<Option<isize>> = vec![None; self.edges.len() + extension];
        let old_row_length = self.nodes.len() - 1;
        for i in (0..(vec.len() - extension)).rev() {
            let y = (i - (i % old_row_length)) / old_row_length;
            vec[i + y] = self.edges[i];
        }
        self.edges = vec;
        self.nodes.len() - 1
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<isize> {
        self.edges[(from * self.nodes.len() + to)]
    }

    pub fn set_mono_edge(&mut self, from: usize, to: usize, weight: isize) {
        self.edges[(from * self.nodes.len() + to)] = Some(weight);
    }

    pub fn set_bi_edge(&mut self, from: usize, to: usize, weight: isize) {
        self.edges[(to * self.nodes.len() + from)] = Some(weight);
        self.edges[(from * self.nodes.len() + to)] = Some(weight);
    }
}

impl<T> Debug for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        let mut result = String::from("\n");
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes.len() {
                result.push_str(match self.get_edge(x, y) {
                    Some(_) => "0",
                    None => "X",
                });
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

impl<T> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        let mut result = String::from("\n");
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes.len() {
                result.push_str(match self.get_edge(x, y) {
                    Some(_) => "✅",
                    None => "🟥",
                });
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

pub struct Node<T> {
    pub element: T,
    visited: bool,
}

impl<T> Node<T> {
    pub fn set_visited(&mut self) {
        self.visited = true;
    }
}

impl<T> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "Node`{{`..., {}`}}`", self.visited)
    }
}
