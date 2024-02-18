fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut result = v
        .iter()
        .map(|s| s.len())
        .map(|i| Box::new(i))
        .collect::<Vec<Box<usize>>>();

    result.push(Box::new(
        v.iter().map(|s| s.len()).reduce(|l, r| l + r).unwrap_or(0),
    ));

    result
}

#[cfg(test)]
mod basicbox_sum_tests {
    use super::*;

    #[test]
    fn test_basicbox_sum() {
        let v = vec!["hello".to_string(), "world".to_string()];
        let result = basicboum(v);
        assert_eq!(vec![Box::new(5), Box::new(5), Box::new(10)], result);
    }
}
