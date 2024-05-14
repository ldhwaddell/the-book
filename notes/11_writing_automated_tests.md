# Writing Automated Tests

## 11.1 How to Write Tests

- Tests typically perform:
  - Set up any needed data or state.
  - Run the code you want to test.
  - Assert the results are what you expect.
- Because `tests` is an inner module, we need to bring the code under tests in the outer module into the scop of the inner module. This is done using `use super::*;`

#### The Anatomy of a Test Function

- A test is a function that is annotated with the test attribute `#[test]`
- Can pass an arg to `cargo test` telling it to only run tests whos name matches the string.
- Tests fail when something in the test panics.

#### Checking Results with the `assert!` Marco

- Give it an arg that evaluates to a bool. If true, test pass, else panic! and fail.

#### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

- The order of arguments does not matter.
- Rust calls them `left` and `right`.
- For structs and enums we create, we need to implement `PartialEq` to assert equality. Usually just add `#[derive(Debug, PartialEq)]`

#### Adding Custom failure Messages

- Just add after the main argument for the assertion

#### Checking for Panics with `should_panic`

- Test passes if a panic is detected
- Use #[should_panic] attribute
- We can add the expected param to ensure that the failure message contains the provided text. Does a 'contains' check. Could also specify the entire message depending on the context.

#### Using `Result<T, E>` in Tests

- Use `Result` type instead of assert
- To assert that test should fail, dont use `?` on the result value, use `assert!(value.is_err())`

## 11.2 Controlling How Tests are Run

- Default behavour is to run every test in parallel
- Must make sure tests don't depend on each other or any shared state.
- Use `cargo test -- --test-threads=1` to un on one thread

#### Showing Function Output

- To run tests and show the function prints and output, use `cargo test -- --show-output`

#### Running a Subset of Tests by Name

- Pass thename of the test to only run that one test
- We can specify part of a test name, any tests that match that value will be ran.
- Can run all the tests in a module by filtering on the modules name: `cargo test <mod_name>`

## Ignoring Tests Unless Specifically Requested
- Add `#[ignore]` attribute
- Run `cargo test -- --ignored` to run ignored tests. 
- To run all `cargo test -- --include-ignored`

## 11.3 Test Organization
- Unit tests: small, focused, testing one module at a time. Can test private interfaces
- Integration tests external to library and use it in the same way external code would. Using only public interface. 

#### Unit Tests
- Put them in `src/` in each file with the code they're testing.
- Convention to create a module names tests in each file to contain the functions and annotate with `#[cfg(test)]`
- `#[cfg(test)]` means only compile and run on `cargo test`, not build. 
- nothing to stop you from testing private functions. 

#### Integration Tests
- External to library. 
- Create `tests/` dir:
```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
- Each file in `tests/` is a separate crate, so we need to bring out library into each test crates scope. 
- Don't need to annotate with `#[cfg(test)]`
- If any section fails, the following sections won't be run. 
- Can still run a specific test: `cargo test --test integration_test`


#### Submodules in Integration Tests
- Because each file is treated as a separate crate, we can't just have a file with like a 'setup' function that the other test files use. 
- To get around this:
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
- This tells rust not to treat `common` module as an integration test file. 
- We can then use commmon from any of the files as a module. 
- 


