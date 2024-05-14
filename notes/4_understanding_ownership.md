# Understanding Ownership

- Ownership lets Rust make memory safe guarantees without needing a garbage collector.

## 4.1 What is Ownership?

- Ownership is the rules that govern how a Rust program manages memory.

  ### The Stack and the Heap

  - Both parts of memory available to code at runtime
  - Stack stores values in the order it gets them, removes in opposite order. (LIFO)
  - Data stored on stack must have known, fixed size.
  - Data with unknown size at compile time, or size that might change needs to be stored on the heap.
  - When you put data on the heap, you request a certain amount of space. Mem allocator finds an empty spot big enough for the request, and returns a pointer with the address of that location. (allocating on the heap).
  - The pointer is a known, fixed size so it can be stored on stack, but its referenced is on the heap.
  - Pushing to stack is faster than allocating on heap.
  - When code calls a func, the values passed to the func and functions local variables get pushed onto stack. When func finishes, they get popped from stack.

- Ownership rules

  - Each value in Rust has an owner
  - There can only be one owner at a time
  - When the owner goes out of scope, the value will be dropped.

- Variable Scope

  - Scope is the range within a program for which an item is valid.
  - Variable is valid from the time it is declared until the end of the current scope:

  ```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
  ```

- The `String` type

  - Manages data allocated on the heap and it able to store an amount of text unknown at compile time:
    `let s = String::from("hello");`
  - `::` operator lets us namespace the from function under the `String` type.
  - The string _can_ be mutated:

  ```rust
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
  ```

- Memory and Allocation

  - So that the `String` type can support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, meaning:
    - The memory must be requested from the memory allocator at runtime
    - We need a way of returning this memory to the allocator when done with the `String`
  - Memory is automatically returned once the variable that owns it goes out of scope. Rust calls a function called `drop`

- Variables and Data interacting with Move

  - Consider:

  ```rust
    let s1 = String::from("hello");
    let s2 = s1;
  ```

  - What's actually happening:
    - String has 3 parts; pointer to the memory that holds the contents of the string, a length, and a capacity. This part is stored on the stack (4-1):
      ![String memory allocation](./assets/4/4-1.svg)
    - The length is how much memory, in bytes, that the string is using. The capacity os the total amount, in bytes, that the string received from the allocator.
  - When `s1` is assigned to `s2`, the `String` data is copied. The pointer, length and capacity are copied on the stack. But the data on the heap is not copied (4-2):
    ![String copying](./assets/4/4-2.svg)
  - We know that when variable goe sout of scope, Rust calls `drop` to clean up the heap for that variable. But 4-2 shows how both pointers point to the same location. When they go out of scope, Rust would try to free the same memory twice leading to an error.
  - To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` invalid. Because rust invalidates `s1`, we say that `s1` was **moved** to `s2`.
  - Rust never automatically creates a 'deep' copy of data. So any automatic copying can be assumed to be inexpensive.

- Variables and Data interacting with Clone

  - If we do want to deeply copy the heap data of `String`, we can use `clone`:

  ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
  ```

- Ownership and Functions

  - Passing a variable to a function will move or copy, just as assignment does:

  ```rust
    fn main() {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
  ```

- Return Values and Scope

  - Returning values can also transfer ownership:

  ```rust
    fn main() {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {             // gives_ownership will move its
                                                // return value into the function
                                                // that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string                              // some_string is returned and
                                                // moves out to the calling
                                                // function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope

        a_string  // a_string is returned and moves out to the calling function
    }
  ```

  - Variable ownership follows the same pattern: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, `drop` is called to clean it up, unless ownership of that data has been moved to another variable.

## 4.2 References and Borrowing

- A `reference` is like a pointer in that it's an address we can follow to access the data stored at that address, where that data is owned by another variable. Unlike a pointer, a reference is guaranteed to point to a valid value of some type for the life of that reference.
- This function has a reference to an object as a param, instead of taking ownership:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

- `&` means reference, allowing you to refer to value without ownership.
- This shows `&String s` pointing to `String s1`:
  ![String reference](./assets/4/4-5.svg)
- When functions have refs as params instead of actual values we don't need to return values in order to give back ownership, because we never had it.
- We call the action of creating a reference `borrowing`.
- References are mutable by default.

- Mutable references

  ```rust
    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
  ```

  - If we have a mutable reference to a value, we can have no other references to that value. Code that tries to create two mutable references to a value will fail.
  - This restriction helps prevent data races at compile time. Data race means:
    - 2 or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data
    - There's no mechanism being used to synchronize access to the data.
  - We can use curly braces to create a new scope, allowing multiple mutable references, just not simultaneous ones.
  - Can't have mutable reference while we have immutable one to the same value.
  - This is allowed cuz the last usage of the immutable references happens before we introduce the mutable reference:

  ```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
  ```

- Dangling References

  - Rust compiler guarantees that references will never be dangling references. If you have a reference to some data, the compiler ensures that the data will not go out of scope before the ref to the data does.
  - Can't do this:

  ```rust
    fn dangle() -> &String { // dangle returns a reference to a String

        let s = String::from("hello"); // s is a new String

        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!
  ```

### Rules of References

- At any given time, you can have either one mutable reference, or any number of immutable references.
- References must always be valid.

## 4.3 The Slice Type

- Slices let you reference a contiguous sequence of element in a collection, rather than the whole collection.
- Slices are types of references, so they do not have ownership.
- Suppose we had this function:

  ```rust
      fn first_word(s: &String) -> usize {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return i;
          }
      }

      s.len()
    }
  ```

  - Issue, the returned `usize`
    is only relevant in the context of the `&String`. Because it is separate from the `String`, there is no guarantee that it will be valid in the future:

    ```rust
        fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
        }
    ```

- String Slices

  - A reference to part of a string:

  ```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
  ```

  - For example, `world` is a slice that contains a pointer to the byte at index 6 with a length value of 5:
    ![String slice](./assets/4/4-6.svg)
  - A Function to return the first word of string:

  ```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
  ```

  - String slices as parameters

    - Can improve the function by passing a string slice:

    ```rust
    fn first_word(s: &str) -> &str {
    ```

    - With this, if we have a string slice we can pass directly. If we have a string, we can pass a slice of the string or a reference to the string:

    ```rust
        fn main() {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        }
    ```
