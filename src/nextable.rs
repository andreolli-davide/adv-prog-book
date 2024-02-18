use std::fmt::Debug;

trait Nextable {
    fn gimme_next(&self) -> Option<Self>
    where
        Self: Sized;
}

impl Nextable for i32 {
    fn gimme_next(&self) -> Option<Self> {
        Some(*self + 1)
    }
}

impl Nextable for char {
    fn gimme_next(&self) -> Option<Self> {
        if *self == 'z' {
            None
        } else {
            Some((*self as u8 + 1) as char)
        }
    }
}

fn printnext(nextable: &(impl Nextable + Debug)) {
    println!("next of {:?} is {:?}", nextable, nextable.gimme_next())
}

#[cfg(test)]
mod nextable_tests {
    use super::*;

    #[test]
    fn test_nextable_i32() {
        let i = 42;
        assert_eq!(i.gimme_next(), Some(43));
    }

    #[test]
    fn test_nextable_char() {
        let c = 'c';
        let z = 'z';
        assert_eq!(c.gimme_next(), Some('d'));
        assert_eq!(z.gimme_next(), None);
    }
}
