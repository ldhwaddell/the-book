# Enums and Pattern Matching

- Enums let us define a type by enumerating its possible variants

## 6.1 Defining an Enum

- Enums give us a way of saying a value is one of a possible set of values.

```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
```

- Variants of enum are namespaced under its identifier.
- We can put data directly into enum variant:

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

```

- The name of each enum variant we define becomes a fucntion that constructs an instance of the enum.
- Each variant can have different types and associated data:

```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

- Can have variety of types in variants:

```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
```

- The `Option` enum and its advantages over null values

  - For the scenario where a value could be something or nothing

  ```rust
  enum Option<T> {
    None,
    Some(T),
  }
  ```

  - `<T>` syntax is generic type parameter. Means that the Some variant of Option can hold one piece of data of any type:

  ```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
  ```

  - You have to convert an `Option<T>` to a `T` before you can perform `T` operations.

## 6.2 The `match` Control Flow Construct

- First pattern the value fits is a match:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- With `if`, the expression needs to evaluate to a boolean. `match` can be any type.
- match arms can bind to parts of the values that match the pattern. This lets us extract values out of enum variants:

```rust
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

- Matching with `Option<T>`

  - Suppose we want a function that takes an `Option<i32>` and add 1 to the value if there is one:

  ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
  ```

- Matches must be Exhaustive

  - The arms' patterns must cover all possibilities
  - To bind to other values:

  ```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
  ```

  - To not use the value in the catch all:

  ```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
  ```

  - So nothing happens in the catch all

  ```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
  ```

## 6.3 Concise Control Flow with `if let`

- Lets you combine the terms to handle values that match one pattern while ignoring the rest.
- Can be with or without `else`:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
