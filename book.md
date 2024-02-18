# Advanced Programming Book
By Davide Andreolli

## Gimme Double \(2022/11\)
> Define the `Doublable` trait with a method `gimme_double` implement `Doublable` for `i32`, `gimme_double` returns a new `i32` that is twice self implement `Doublable` for `String`, `gimme_double` returns a new `String` that is self concatenated with self implement a function `printdouble` that takes a `Doublable` and prints the argument and its `gimme_double` using the `":?"` formatter it behaves as the example: _doubling 5 is 10 doubling "what" is "whatwhat"_.

```rust
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
```

## Wrapper for i32 odds \(2022/11\)
> Define a struct `Wrapper` that contains a field `v` of type `Vec<i32>` define an iterator for `Wrapper` to cycle over the elements of the vector the iterator will skip every other element, effectively accessing only those at odd index in the inner vector (the first element is at index 0)

```rust
#[derive(Clone)]
struct Wrapper {
    v: Vec<i32>
}

impl Iterator for Wrapper {

    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(number) = self.v.clone().get(1) {
            self.v = self.v[2..].to_vec();
            Some(*number)
        }
        else {
            None
        }
    }
}

impl Wrapper {
    fn iter(&self) -> impl Iterator<Item = i32> {
        self.clone()
    }
}
```
