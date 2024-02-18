#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct Content {
    s: String,
    b: bool,
    i: i32,
}

impl Content {
    pub fn new_with(s: String, b: bool, i: i32) -> Content {
        return Content { s, b, i };
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, len: 0 }
    }

    fn get(&self, mut pos: i32) -> Option<&T> {
        let mut head = self.head.as_ref();
        while pos > 0 && head.is_some() {
            head = head.unwrap().next.as_ref();
            pos -= 1;
        }
        if let Some(head) = head {
            Some(&head.elem)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> List<T> {
    fn add(&mut self, elem: T) {
        let mut current = &mut self.head;

        while current.as_ref().map_or(false, |node| elem > node.elem) {
            current = &mut current.as_mut().unwrap().next;
        }

        let new_node = Box::new(Node {
            elem,
            next: current.take(),
        });

        *current = Some(new_node);
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
mod list_tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List::new();
        list.add(Content::new_with("world".to_string(), false, 2));
        list.add(Content::new_with("hello".to_string(), true, 1));
        list.add(Content::new_with("!".to_string(), true, 3));
        println!("{:?}", list);
        assert_eq!("hello".to_string(), list.get(0).unwrap().s);
        assert_eq!("world".to_string(), list.get(1).unwrap().s);
        assert_eq!("!".to_string(), list.get(2).unwrap().s);
    }
}
