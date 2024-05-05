# Error Handling

- 2 Major categories
  - Recoverable
  - Non-recoverable
- Rust does not have exceptions.
- Uses the `Result<T, E>` for recoverable errors
- Uses `panic!` for unrecoverable errors.

## 9.1 Unrecoverable Errors with panic!

- By default panics print a failure message, unwind, clean up the stack, and quit.
- Can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what caused the error.
- Key to readig backtrace is you start at the top and read until you see files you wrote.

## 9.2 Recoverable Errors with `Result`

- Result enum:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `T` represents the type of value that will be returned in a success case.
- `E` represents the type of error that will be returned in a fail case.
- Handling with match:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

- Matching different errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

- Using closures to clean up many `matches`:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

- Shortcuts for Panic on error

  - `unwrap`: If `Ok`, return the value inside. Else, it will call the panic macro.
  - In production, usually best to use `.expect(msg)` and give details about what caused the error
  -

- Propagating Errors

  - The verbose way:

  ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
  ```

  - Using the `?` operator:

  ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
  ```

  - The `?` operator is very similar to the `match` above, but values go through the `From` function. The error type gets converted to the error type defined in the return type of the current function.
  - Chaining `?`:

  ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
  ```

  - Or built in method:

  ```rust
    use std::fs;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
  ```

  - `?` operator can only be used in functions who's return type is compatible with the value `?` is used on.
  - Could be used with `Option`:

  ```rust
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
  ```

## 9.3 To `panic!` or Not to `panic!`
- Panic is good in examples, prototypes, and tests. 
- If you can ensure you will never have `Err` variant, it is still ok to call `unwrap`:
```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```
- When to panic:
  - The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
  - Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
  - There’s not a good way to encode this information in the types you use
- When failure is expected, more appropriate to return a `Result`
