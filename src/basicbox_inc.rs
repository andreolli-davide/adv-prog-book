fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    v.iter()
        .map(|s| s.len() + 1)
        .map(|i| Box::new(i))
        .collect::<Vec<Box<usize>>>()
}

#[cfg(test)]
mod basicbox_sum_tests {
    use super::*;

    #[test]
    fn test_basicbox_sum() {
        let v = vec!["hello".to_string(), "world".to_string(), "".to_string()];
        let result = basicbox_sum(v);
        assert_eq!(vec![Box::new(6), Box::new(6), Box::new(1)], result);
    }
}
