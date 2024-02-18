use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub s:String
}

impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T> Node<T> {
    pub fn new(elem:T) -> Node<T> {
        Node {
            elem,
            left:None,
            center:None,
            right:None
        }
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
    size : i32,
}

type TreeLink<T> = Option<Box<Node<T>>>;

impl<T> Tree<T> {

    fn new() -> Self {
        Self {
            root: None,
            size: 0
        }
    }
}

impl<T: PartialOrd> Tree<T> {
    
    fn add_node(&mut self, el: T) -> () {
        let mut current = &mut self.root;
        while let Some(node) = current {
            current = match el.partial_cmp(&node.elem).unwrap() {
                Ordering::Less => &mut node.left,
                Ordering::Equal => &mut node.center,
                Ordering::Greater => &mut node.right
            }
        }
        *current = Some(Box::new(Node::new(el)));
        self.size += 1;
    }

    fn howmany_smaller(&self, el: &T) -> i32 {
        if let Some(root) = self.root.as_ref() {
            let mut count = 0;
            let mut queue = VecDeque::new();
            queue.push_back(root.as_ref());

            while let Some(current) = queue.pop_front() {
                if &current.elem < el {
                    count += 1;
                }
                if let Some(node) = current.left.as_ref() {
                    queue.push_back(node);
                }
                if let Some(node) = current.center.as_ref() {
                    queue.push_back(node);
                }
                if let Some(node) = current.right.as_ref() {
                    queue.push_back(node);
                }
            }

            count
        }
        else {
            0
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.s.len().partial_cmp(&other.s.len())
    }
}

#[cfg(test)]
mod tree_partialord_tests {
    use super::*;

    #[test]
    fn test_tree_partialord() {
        let mut t = Tree::new();
        t.add_node(Content::new(1, "hello".to_string()));
        t.add_node(Content::new(2, "worldyx".to_string()));
        t.add_node(Content::new(3, "".to_string()));
        assert_eq!(2, t.howmany_smaller(&Content::new(0, "worldy".to_string())));
    }
}
