# Common Collections

- Vector lets you store variable number of values next to each other
- string is collection of characters
- hash map lets you associate a value with a particular key.

## 8.1 Storing Lists of Values with Vectors

- Puts values next to each other in memory.
- Can only store values of the same type
- Creating vecs:

```rust
// Without default values
let v: Vec<i32> = Vec::new();

// With default values, uses vec! macro
let v = vec![1, 2, 3];
```

- To create a vector and add elements we can use `v.push(4)`. But must declare vec as mutable.
- Elements can be read by index or with `.get`. Invlaid index causes panic, invalid index with `.get` returns None.
- Iterating
  - immutable refs:
  ```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
  ```
  - mutable refs:
  ```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
  ```
- Can store multiple types in a vec with an enum.
- Dropping a vector drops its elements.

## 8.2 Storing UTF-8 Encoded Text with Strings

- Strings are implemented as a collection of bytes
- In the core library, Rust has string slice `str`, usually seen in borrowed form `&str`.
- The `String` type, provided by Rust's standard library, is a growable, mutable, owned, UTF-8 encoded string type.
- `String` has many of the same operations as `Vec<T>`, cuz it is implemented as a wrapper around a vector of bytes.
- Creating a string:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

let s = String::from("initial contents");
```

- Updating a string:

```rust
// Does not take ownership
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");

let mut s = String::from("lo");
s.push('l');

// concatenate with +
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.

// Using references so ownership does not get taken of any of the params:
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
```

- Indexing:

  - Can't just access part of string by index. The bytes required for a 'letter' might not be just in one index of the vec.

- Slicing Strings

```rust
// 2 bytes for each char
let hello = "Здравствуйте";

let s = &hello[0..4];
// s is Зд

// But, this will error
let s2 = &hello[0..1];

```

- Iterating

```rust
// Pick if you want chars:
for c in "Зд".chars() {
    println!("{c}");
}

// or Bytes
for b in "Зд".bytes() {
    println!("{b}");
}
```

- Valid unicode scalar may be made up of more than one byte.

## 8.3 Storing Keys with Associated Values in Hash Maps

- Creating a hashmap

  - Data is stored on the heap.
  - All keys must have same type
  - All values must have same type

  ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
  ```

- Accessing values in Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

// Can iterate with for loop
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

- Hash Maps and Ownership

  - For types that implement a `Copy` trait , they get copied into the hashmap. For owned values like `String`, the values get moved and the hash map is the owner.
  - if we insert references, the values won't be moved into the hash map. But we need to make sure they are valid for at least as long as the hash map.

- Updating a hashmap

  - Each key can only have one value at a time
  - Overwriting:

  ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
  ```

  - Add key only if key isn't present:

  ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
  ```

  - Updating value based on old value

  ```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
  ```

  - 
