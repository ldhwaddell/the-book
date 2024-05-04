# Using Structs to Structure Related Data

## 5.1 Defining and Instantiating Structs

- Structs are similar to tuples, they both hold related values.
- Pieces of struct can be diff types, but you need to name each piece:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- To use Struct after its been defined, we create an instance of that struct by specifying values for each of the fields:

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

- Use dot notation to get a field from the struct.
- To change a field, the whole Struct instance must be marked mutable.
- A function to build return structs, using Field Init shorthand:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

- Struct Update Syntax
  - Let's us create a new struct, based on values of an existing one:
  ```rust
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
  ```
  - But, this uses `=` like an assignment, so it moves the data. This means that we can no longer use `user1` after creating `user2` because the `String` in the `username` field was moved to `user2`. 

- Using Tuple Structs Without Named Fields to Create Difference Types
  - Tuple Structs don't have names associated with their fields. 
  ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
  ```

- Unit-Like Structs Without Any Fields
  - Behave similarly to `()`
  - Useful when we need to implement a trait on some type, but don't have anydata to store in the type itself. 
  ```rust
    struct AlwaysEqual;

    fn main() {
        let subject = AlwaysEqual;
    }
  ```

## 5.3 Method Syntax
- Methods are defined in the context of a struct, enum, or traint object. 
- First param is always self, the instance of the truct the method is being called on. Within the Impl block, the type Self is an alias for the type that the impl block is for. 
- Methods can take ownership of self, and can borrow mutably or immutably.

- Associated Functions
  - All functions defined inside an impl block are called associated functions because they are associated with the type after the impl. 
  - We can define associated functions that don't have self as the first parameter (so they aren't methods), if they do not need an instance of the type to work. 
  - Often used to return a new instance of a struct. 
  - Could create a square:
  ```rust
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
  ```
  - To call this, we would use `let sq = Rectangle::sqaure(3)`. Meaning it is namepsaced by the struct. 
