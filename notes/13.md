# Functional Language Features: Iterators and Closures

- Closure: Function like construct you can store in a variable
- Iterator: Way of processing series of elements.

## 13.1 Closures: Anonymous Funcitons that Capture their Environment

- Closures can capture values from the scope in which they're defined.

#### Capturing the Environment with Closures

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

- `unwrap_or_else` takes is passed the 0 param closure `|| self.most_stocked()`
- We passed a closure that calls `self.most_stocked()` on the current `Inventory` instance. It captures an immutable reference to the `self` `Inventory` and passes it to the `unwrap_or_else`

#### Closure Type Inference and Annotation

- There are all valid

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

- But this would not be, the compiler locks on to the type of the first instance the closure was called on:

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

#### Capturing References or Mobing Ownership

- Borrowing Immutably

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

- Borrowing Mutably:

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
```

- Forcing closure to take ownership of values:

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

- Mostly useful when passing a closure to a new thread to move the data so that it's owned by the new thread.

#### Moving Captured Values out of Closures and the `Fn` traits

- A closure can:
  - Move a captured value out of the closure
  - mutate the captured value
  - neither move nor mutate the value
  - Capture nothing from the environment
- `Fn` traits:
  - `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits, because it can only be called once.
  - `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
  - `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

```rust
// Implements `FnOnce`
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

- Functions can implement all of the `Fn` traits.

```rust
// Implements `FnOnce` because closure can be called more than once. One time in each item slice.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

## 13.2 Processing a Series of Items with Iterators

- Iterators are `lazy`. They have no effect until you call methods that consume the iterator.

#### The `Iterator` Trait and the `next` Method

- All iterators implement the `Iterator` trait:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

}
```

- `type Item` and `Self::Item` define an associated type with the trait. This says that implementing the `Iterator` trait requires we also define an `Item` type, and the `Item` type will be used in the return type of `next`.
- The `Iterator` trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.
- Each call to `next` eats up an item in the iterator.
- Values returned from `next` are immutable references to values in the vector.
- To create an iterator that takes ownership and returns owned values, call `into_iter`. It iterate over mutable references, call `iter_mut`.

#### Methods that Consume the Iterator

- Methods that call `next` are called consuming adaptors cuz calling them uses up the iterator.

#### Methods that Produce Other Iterators

- Iterator adapters are defined on the `Iterator` trait that don't consume the iterator. They produce different iterators by changing some aspect of the original iterator.

#### Using Closures that Capture the Environment

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```
