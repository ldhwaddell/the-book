# Managing Growing Projects with Packages, Crates, and Modules

- Package can contain multiple binary crates and optionally one library crate.
- Packages: Cargo feature that lets you test, build, share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, privacy of paths
- Paths: A way of naming an item such as struct, function, module

## 7.1 Packages and Crates

- Crate is the smallest amojnuf of code that the Rust compiler considers at a time.
- Binary crates are programs you can compile into executable
- Library crates don't have main function, don't compile to executable. Interchangable with library.
- Package is bundle of 1 or more crates that provide a set of functionality.
- Package can contain as many binary crates as needed, but only one library crate.
- Package can have multiple binary crates by placing files in the `src/bin` director: each file is a separate binary crate.

## 7.2 Defining Modules to Control Scope and Privacy

#### Modules cheat Sheet

- Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
- Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file src/garden.rs
  - In the file src/garden/mod.rs
- Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file src/garden/vegetables.rs
  - In the file src/garden/vegetables/mod.rs
- Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with use `crate::garden::vegetables::Asparagus`; and from then on you only need to write `Asparagus` to make use of that type in the scope.

- Grouping Related Code in Modules
  - Modules let us organize code within a crate.
  - Allow us to control the privacy of items.
  - Modules can be placed inside of modules
  - `src/main.rs` and `src/lib.rs` are called crate roots. The content of either of these files form a module named `crate` at the root of the module structure, known as the module tree.

## 7.3 Paths for Referring to an Item in the Module Tree

- To call a function, we need to know its path.
- Path can be:
  - Absolute path: Full path starting from crate root. For code from external crate it starts with the crate name. For code from current crate, it starts with the literal `crate`.
  - Relative path: Start from current module and uses `self`, `super`, or an identifier in the current module.
- Generally better to use absolute paths
- All items are private to the parent modules by default. If you want to make an item like a func or struct private, put it in a module.
- Items in a parent can't use the private items inside child modules, but items in child modules can use the items in the ancestor modules.
- Making module public does not make its content public. `Pub` on module only lets code in ancestor modules refer to it.
- Can make relative paths that begin in the parent module, rather than current module or crate root using `super`:

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

- If we use pub before a struct, we make the struct public, but it's fields are still private.
- Default for enum is for every variant to be public, structs everything private.

## 7.4 Bringing Paths Into Scope with the `use` Keyword

- Can use `use` to bring something into scope.
- Kind of like creating a symbolic link in the filesystem.
- `use` only creates the shortcut in the scope that the `use` occurrs in.
- Leave parent module name with `use`, but idiomatic to use full path when getting structs, enums, and other items:

```rust
//mod
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// struct:
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- Can rename with `as`:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

- When we bring a name into scope with `use`, the name is private. To make it public, use `pub use`, which is called _re-exporting_
- Can use nested paths to combine lists of `use`:

```rust
// Before
use std::cmp::Ordering;
use std::io;

// After
use std::{cmp::Ordering, io};

// Before
use std::io;
use std::io::Write;

// After
use std::io::{self, Write};
```

- Can bring all public items into scop with glob operator:

```rust
use std::collections::*;
```

## 7.5 Separating Modules into Different Files

- Only need to load a file using `mod` declaration once in the program.
- For a module named `front_of_house` declared i nthe crate root, the compiler looks in:
  - src/front_of_house.rs
  - src/front_of_house/mod.rs (older style, still supported path)
- For a module named `hosting` that is a sub mobule of `front_of_house`, the compiler looks in:
  - src/front_of_house/hosting.rs
  - src/front_of_house/hosting/mod.rs (older style, still supported path)
