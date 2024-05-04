# Common Programming Concepts

## 3.1 Variables and Mutability

- Variables immutable by default.
- Constants
  - Can't use `mut` with constants
  - Type of value must be annotated.
  - Can be declared in any scope.
  - Can only de set to a constant expression, not the result of value that could only be computed at runtime.
  - Naming convention ALL_CAPS_WITH_UNDERSCORES
- Shadowing
  - Can shadow a variable by using the same variable name and the `let` keyword.
  - Not the same as marking a variable as mut.
  - Lets us perform transformations but have the variable be immutable
  - Effectively creating a new variable when we use `let` again.
  - Can mutate value, not type, so name shadowing good for converting types.

## 3.2 Data Types

- Scalar Types
  - Integer
    - | Length  | Signed | Unsigned |
      | :-----: | :----: | :------: |
      |  8-bit  |   i8   |    u8    |
      | 16-bit  |  i16   |   u16    |
      | 32-bit  |  i32   |   u32    |
      | 64-bit  |  i64   |   u64    |
      | 128-bit |  i128  |   u128   |
      |  arch   | isize  |  usize   |
    - Signed numbers stored in 2
      s complement
  - Floating-Point
    - `f32`: single precision
    - `f64`: double precision
  - Boolean
    - One byte in size
  - Character Type
    - Specify `char` with single quote '', string with double quotes ""
    - `char` is 4 bytes in size
  - Compund Types - Group multiple values into one type.
  - Primitives: Tuple, Array
    - Tuple:
      - Fixed length
      - `let tup: (i32, f64, u8) = (500, 6.4, 1);`
      - To access the ith element: `let x = tup.i`
      - Tuple without values is called a unit
    - Array
      - Every element must have same type. 
      - Useful when you want data on the stack instead of the heap. 
      - Not as flexible as vector type
      - Fixed number of elements
      - `let a: [i32; 5] = [1, 2, 3, 4, 5];`
      - Array is single chunk of memory with a known, fixed size. 
## 3.3 Functions
- Must declare type of each parameter. 
- `Statements` are instructions that perform some action and do not return a value
  - `ley y = 6;`
  - Function definitions
  - Do not return values
- `Expressions` evaluate to a resultant value
  - `5+6` evaluates to 11
  - Calling a function, macro, a new block scope from {}
  - Expressions do not have ending semicolon. Adding semicolon to expression makes it a statement
- Functions with return values
  - Don't name them, but must declare type. 
## Control Flow
- `if`
  - Condition must be a `bool`
  - Rust does not automatically convert to bool
- `if` in `let`
  - Must have same types
- Loops
  - Can use loop labels to distinguish nested loops
