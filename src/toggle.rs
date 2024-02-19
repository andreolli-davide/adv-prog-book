use std::fmt::Debug;

trait Toggle {
    fn toggle(&mut self);
}

impl Toggle for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

impl Toggle for i32 {
    fn toggle(&mut self) {
        *self = -*self;
    }
}

impl Toggle for String {
    fn toggle(&mut self) {
        *self = self
            .chars()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect::<String>();
    }
}

fn toggle_and_print<T: Toggle + Debug + Clone>(value: &T) {
    let mut cloned = value.clone();
    cloned.toggle();
    println!("{:?} toggled is {:?}", value, cloned);
}

#[cfg(test)]
mod toggle_tests {
    use super::*;

    #[test]
    fn test_toggle_bool() {
        let mut value = true;
        toggle_and_print(&value);
        value.toggle();
        assert_eq!(value, false);
    }

    #[test]
    fn test_toggle_i32() {
        let mut value = 42;
        toggle_and_print(&value);
        value.toggle();
        assert_eq!(value, -42);
    }

    #[test]
    fn test_toggle_string() {
        let mut value = "Hello, World!".to_string();
        toggle_and_print(&value);
        value.toggle();
        assert_eq!(value, "hELLO, wORLD!");
    }
}
