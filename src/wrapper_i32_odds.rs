#[derive(Clone)]
struct Wrapper {
    v: Vec<i32>,
}

impl Iterator for Wrapper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(number) = self.v.clone().get(1) {
            self.v = self.v[2..].to_vec();
            Some(*number)
        } else {
            None
        }
    }
}

impl Wrapper {
    fn iter(&self) -> impl Iterator<Item = i32> {
        self.clone()
    }
}

#[cfg(test)]
mod wrapper_i32_odds_tests {
    use super::*;

    #[test]
    fn test_wrapper_i32_odds() {
        let w = Wrapper {
            v: vec![1, 2, 3, 4, 5, 6],
        };
        let mut iter = w.iter();
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(None, iter.next());
    }
}
