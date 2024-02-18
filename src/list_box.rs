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
    fn remove(&mut self, mut pos: i32) -> Result<(), String> {
        if pos < 0 || pos >= self.len {
            return Err("wrong position".to_string());
        }

        let mut current = &mut self.head;

        while pos > 0 && current.is_some() {
            current = &mut current.as_mut().unwrap().next;
            pos -= 1;
        }

        let next = current.as_mut().unwrap().next.take();
        *current = next;
        self.len -= 1;

        Ok(())
    }

    fn pop(&mut self) -> Result<(), String> {
        self.remove(0)
    }

    fn pop_last(&mut self) -> Result<(), String> {
        self.remove(self.len - 1)
    }

    fn get(&self, mut pos: i32) -> Option<&T> {
        if pos < 0 || pos >= self.len {
            return None;
        }

        let mut current = &self.head;

        while pos > 0 {
            current = &current.as_ref().unwrap().next;
            pos -= 1;
        }

        Some(&current.as_ref().unwrap().elem)
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List {
            head: Some(Box::new(Node {
                elem: Content::new_with("hello".to_string(), true, 1),
                next: Some(Box::new(Node {
                    elem: Content::new_with("world".to_string(), false, 2),
                    next: None,
                })),
            })),
            len: 2,
        };

        assert!(list.remove(1).is_ok());
        assert_eq!(list.len, 1);
        assert_eq!(list.get(0).unwrap().s, "hello");
        assert!(list.remove(0).is_ok());
        assert_eq!(list.len, 0);
    }
}
