## 2. Programming a Guessing Game
- Variables are immutable by default
- Add `mut` to make variable mutable:
    ```
    let apples = 5; // immutable
    let mut bananas = 5; // mutable
    ```
- Given the line;
    ```
    let mut guess = String::new();
    ```
    - :: syntax means that new is an associated function of the String type. Associated function is a function that is implemented on a type. 

- ```.read_line(&mut guess)```
    - Take whatever the user types into stdin and append into a string. 
    - & indicates that the argument is a reference. References let multiple parts of the code access one piece of data without needing to copy it into memory multiple times. References are immutable by default. 

- `.read_line` puts whatever the user enters into the string we pass to it, but also returnsx `Result` value.
    - `Result` is an enumeration. A type that can be indifferent states. Each state is called a variant. 

- use `cargo doc --open` to get the docs for crates in the project
- 'shadowing' lets us reuse a variable name rather than creating two unique vars. Often used when converting the type of a value. 
- 