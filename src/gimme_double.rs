use std::fmt::Debug;

trait Doublable {
    fn gimme_double(&self) -> Self;
}

impl Doublable for i32 {
fn gimme_double(&self) -> i32 {
        self * 2
    }
}

impl Doublable for String {
    fn gimme_double(&self) -> String {
        format!("{}{}", self, self)
    }
}

fn print_double<T: Doublable + Debug>(x: T) {
    println!("doubling {:?} is {:?}", x, x.gimme_double());
}

#[cfg(test)]
mod gimme_double_tests {
    use super::*;

    #[test]
    fn test_gimme_double_i32() {
        assert_eq!(4, 2.gimme_double());
    }

    #[test]
    fn test_gimme_double_string() {
        assert_eq!(String::from("hellohello"), String::from("hello").gimme_double());
    }
}

