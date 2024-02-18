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

## BasicBox Sum \(2022/11\)
> Write a function `basicbox_sum` that takes a vector of Strings and returns a vector of Boxes of usizes the returned vector contains all the lengths of the input vector followed by a final element that sums all the previous lengths

```rust
fn basicboc_sum(v: Vec<String>) -> Vec<Box<usize>> {
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
```

## List with boxes (2022/11)
> Take the following `List` and `Node` structs define these functions and methods for `List`, each one defines how many points it yields 
> - [7] remove: takes a position `p:i32` where to remove the element from the list and it returns a `Result<(),String>` The function removes the node at position `p` or returns the string `"wrong position"` if the list has fewer than `p` elements. That is: removing from position 2 in `[10,20,30]` will return `[10,20]`. Removing from position 3 in `[10,20,30]` will return `Err("wrong position)` removing from position 0 in `[10,20,30]` will return `[20,30]`. 
> - [2] pop: removes the head of the list 
> - [2] pop_last: removes the last element of the list 
> - [4] get: takes a position `p` and returns an optional pointer to the pth T-typed element in the list (That is, a pointer to the element, not a pointer to the `Node`) 
>
> Note: the tests already include the code below, all you need to paste as the answer are the impl blocks and possible imports (use ...).

```rust
// Given code
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct Content {
    s: String,
    b: bool,
    i: i32,
}

impl Content {
    pub fn new_with(s: String, b: bool, i: i32) -> Content {
        return Content { s, b, i };
    }
}


// Exercise code
impl<T> List<T> {

    fn remove(&mut self, mut pos: i32) -> Result<(), String> {

        if pos < 0 || pos >= self.len {
            return Err("wrong position".to_string());
        }

        let mut current = &mut self.head;

        while pos > 0 && current.is_some() {
            current = &mut current.as_mut().unwrap().next;
            pos -= 1;
        }

        let next = current.as_mut().unwrap().next.take();
        *current = next;
        self.len -= 1;

        Ok(())
    }

    fn pop(&mut self) -> Result<(), String> {
        self.remove(0)
    }

    fn pop_last(&mut self) -> Result<(), String> {
        self.remove(self.len - 1)
    }

    fn get(&self, mut pos: i32) -> Option<&T> {
        if pos < 0 || pos >= self.len {
            return None;
        }

        let mut current = &self.head;

        while pos > 0 {
            current = &current.as_ref().unwrap().next;
            pos -= 1;
        }

        Some(&current.as_ref().unwrap().elem)
    }
}
```

