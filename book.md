# Advanced Programming Book
By Davide Andreolli

## Gimme Double
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


