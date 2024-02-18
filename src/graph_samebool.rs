use std::{fmt::{Debug, Formatter}, rc::Rc, cell::RefCell};

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}

impl<T: Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}

struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}

pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}

#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub b:bool
}

impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}

impl SameBool for Content{
    fn samebool(&self, other: &Self) -> bool {
        self.b == other.b
    }
}

impl<T> Graph<T> {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
}

impl<T: SameBool + PartialOrd> Graph<T> {

    fn add_node(&mut self, value: T) -> () {
        let mut new_node = Node {
            inner_value: value,
            adjacent: Vec::new(),
        };

        self.nodes
            .iter()
            .filter(|n| n.borrow().inner_value < new_node.inner_value)
            .for_each(|n| new_node.adjacent.push(Rc::clone(n)));

        let new_node = Rc::new(RefCell::new(new_node));

        self.nodes
            .iter()
            .filter(|n| new_node.borrow().inner_value.samebool(&n.borrow().inner_value))
            .for_each(|n| n.borrow_mut().adjacent.push(Rc::clone(&new_node)));

        self.nodes.push(new_node);
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.i.partial_cmp(&other.i)
    }
}

#[cfg(test)]
mod graph_samebool_tests {
    use super::*;

    #[test]
    fn test_graph_samebool() {
        let mut g = Graph::new();
        g.add_node(Content::new_with(1, true));
        g.add_node(Content::new_with(2, true));

        for node in g.nodes.iter() {
            assert_eq!(node.borrow().adjacent.len(), 1);
        }
    }
}

