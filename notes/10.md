# Generic Types, Traits, and Lifetimes

## 10.1 Generic Data Types

- Type naming convention is UpperCamelCase, opten use `T` for generics.

#### In Function Definitions

- Initial functions:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

- When we use a type parameter name in function signature, we have the declare the type parameters name before we can use it.
- Example:

```rust
fn largest<T>(list: &[T]) -> &T {
```

- Reads: largest is generic over some type `T`. This function has one parameter, `list` which is a slice of values of type `T`. The function returns a reference to a value of the same type, `T`.
- But function might not work for every type. For `largest`, we need to restrict to types that implement the `PartialOrd` trait that allows comparison.

#### In Struct Definitions

- Can define a struct to hold values of any type:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

- But this forces `x` and `y` to be the same type. Can use multiple generic parameters to allow different types.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

#### In Enum definitions

- Can define Enums to hold 1 or multiple generics:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### In Method Definitions

- Methods written with an `impl` that declares the generic ye will be defined on any instance of the tye, no matter what concrete type ends up substitutung the generic.
- Can specify contraints on generic types when defining methods on the type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- This means that other instances of `Point<T>` where `T` is not `f32` won't have this method defined.
- Generic type params are not always the same as those used in that same structs method signatures:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### Performance

- Generic types do not make the program run slower.
- Does `monomorphization` at compile time. No runtime cost.
- This turns generic code into specific code by filling in the concrete tyes that are used when compiled.

## 10.2 Traits

- Traits are similar to Interfaces on other languages
- A traits defines functionality a type has and can share with other types.
- Trait bounds specify that a generic type can be any type that has certain behaviour.

#### Defining a Trait

- Trait definitions are a way to group method signatures together to define a set of behaviours necessary for a purpose.
- Suppose we wanted to make a media aggregator that displayed sumaries of text from various sources, like news and tweets.
- Since we need a summary for each type, we can implement a summary trait:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- Compiler enforces that any tye with the `Summary` trait has the `summarize` function.
- Trait can have multiple methods

#### Implementing a Trait in a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- Must bring trait into scope to use it.
- We can implement a trait on a type only if at least one of the trait or type is local to our crate.
- We can't imlement external traits on external types.

#### Default Implementations

- Can create a default implementation of the trait:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

- Then just need to put `impl Summary for NewsArticle {}` to use it.
- Syntax for implementing is the same as overriding, so don't need to do anything different if there are default traits.
- Defauly implementations can call other methods in the same trait, even if those methods don't have a default implementation.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

- But, is not possible to call the default implementation from an overriding implementation of the same method.

#### Traits as Parameters

- `item` parameter is some type that imlpements the `Summary` trait:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

- This means that in the fn body we can call methods on `item` that come from the `Summary` trait.

#### Trait Bound Syntax

- `impl Trait` is just syntax sugar.
- If we wanted them both to have different types, as long as they implement `Summary` we could do:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

- But if we wanted them to be the same type we would need to do:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

#### Specifying Multiple Trait Bounds with `+`

- To implement 2 traits:

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

- 2 traits with generics:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

#### Clearer Trait Bounds with `where` clauses

- Each generic trait has its own trait bounds, so functions with multiple generic type params can have a lot of trait bound info.
- Instead of this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

- Can do this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

#### Returning Types that Implement Traits

- Can make functions that return types that implement some trait
- But can only use `impl Trait` if youre returing a single type.

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

#### Using Trait Bounds to conditionally Implement Methods

- `Pair<T>` implements the `new` function.
- Next `impl` block, `Pair<T>` only implements `cmp_display` if its inner type `T` implements the `PartialOrd` trait AND the `Display` trait:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

- Can conditionally implement a trait for any type that implements another trait. This is called `blanket implementations`.
- Std lib implements `ToString` on any type that implements `Display`:

```rust
impl<T: Display> ToString for T {
}
```

## 10.3 Validating References with Lifetimes

- Lifetimes ensure that references are valid for as long as we need them.
- Every ref has a lifetime, the scope for which that ref is valid.
- Usually implicit and inferred, but need to be annotated when multiple types are possible.

#### Preventing Dangling References with Lifetimes

- This is not valid. When we try to reference `r` in the print, x has gone out of scope, so it gets removed, so we would be trying to reference memory that was deallocated when x went out of scope:

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

#### The Borrow Checker

- Compares scope to determine whether all borrows are valid.
- The life time of `r` is `'a`, the lifetime of `x` is `'b`:

```rust
// Invalid
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

// Valid
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

- In the valid case, the lifetime of 'b is greater than 'a, so `r` can reference `x`.

#### Generic Lifetimes in Functions

- Suppose we want a function that returns the longest of two strs
- Want the function to take string slices, not strings, so that it does not take ownership of params.

```rust
// Does not compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- Rust does not know if the returned val will be a reference to x or y.
- And we don't know the concrete lifetimes of the references, so we can't look at scopes to determine if the reference we return will always be valid.
- To fix this, we need generic lifetime params.

#### Lifetime Annotation Syntax

- Lifetime annotations do not change how long the references live. They describe the relationships of the lifetimes to multipe references to each other.
- Functions can accept references with any lifetime by specifying a generic lifetime parameter.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

#### Lifetime Annotations in Function Signatures

- Need to declare generic lifetime params inside angle brackets
- To express 'the returned reference will be valid as long as both parameters are valid. This is the relationship between the lifetimes of the params and the return value':

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- The conrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`.
- The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.
- The returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.
- So, this is valid:

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
// result will have the same lifetime as string2
```

- This is not valid

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
// The borrowed value, string2, does not live long enough to be printed
```

#### Thinking in Terms of Lifetimes

- We would not need to specify a lifetime on y if we always returned x:
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
- The lifetime param for the return type needs to match the lifetime for one of the params. 
- Invalid:
```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
- `result` goes out of scope at the end of the function, but we are also trying to return a reference to `result`. Best fix here would be to return an owned data type rather than reference. 

#### Lifetime Annotations in Struct Definitions
- We can define structs to hold references, but we need to add a lifetime annotation to every reference in the struct:
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
- The data in `novel` exists before `ImportantExcerpt`, and `novel` does not go out of scope until after `ImportantExcerpt` goes out of scope. 

#### Lifetime Annotations in Mthod Definitions
- Always need to be declared after `impl` and used after the structs name. 
- In method signatures inside the `impl`, references might be tied to the lifetime of references in the structs fields, or they might be independent. 
- Lifetime param after impl and its use after the type name are required, but don't need to annotate the lifetime of the reference to self:
```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
- 2 input lifetimes:
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

#### The Static Lifetime
- Means the reference can live for the entire duration of the program.
- All string literals have the `'static` lifetime. 
- Most of the time, an error message suggesting `'static` results from attempting to create a dangling reference or a mismatch of available lifetimes. 

#### Generic Type Parameters, Trait Bounds, and Lifetimes together
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- `ann` is of generic tye `T`, which can be filled with any type that implements the `Display` trait. 