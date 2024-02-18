use std::str::Chars;

struct ConsIter<'a> {
    iter: Chars<'a>,
}

struct Wrapper {
    inner: String,
}

impl Wrapper {
    fn iter(&self) -> ConsIter {
        ConsIter {
            iter: self.inner.chars(),
        }
    }
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        while let Some(c) = self.iter.next() {
            if c.is_ascii() && !['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase()) {
                result = Some(c);
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod consiter_tests {
    use super::Wrapper;

    #[test]
    fn consiter_fulltest() {
        let wrapper = Wrapper {
            inner: String::from("aAsc1ii"),
        };

        let iter = wrapper.iter();

        assert_eq!(iter.collect::<String>(), String::from("sc1"));
    }
}
