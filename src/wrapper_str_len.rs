struct Wrapper {
    v: Vec<String>,
}

impl Iterator for Wrapper {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.v.clone().get(0) {
            self.v.remove(0);
            Some(s.len())
        } 
        else {
            None
        }
    }
}

impl Wrapper {
    fn iter(&self) -> Iterator<Item = usize> {
        self.clone()
    }
}

#[cfg(test)]
mod wrapper_tests {
    use super::*;

    #[test]
    fn test_wrapper() {
        let w = Wrapper { v: vec!["hello".to_string(), "world".to_string(), "".to_string()] };
        let result = w.iter().collect::<Vec<usize>>();
        assert_eq!(vec![5, 5, 0], result);
    }
}
