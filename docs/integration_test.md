# Rust Integration Testing Tutorial

## Introduction

Integration tests in Rust are a way to test the various modules of your application in conjunction to ensure they work together as expected. Unlike unit tests, which focus on individual functions or methods, integration tests focus on the application as a whole. This tutorial recaps what we've discussed about setting up and running integration tests in a Rust project.

## Project Structure

A typical Rust project with integration tests has the following structure:

```
your_project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
└── tests/
    └── your_integration_tests.rs
```

## Steps

### 1. Create a `tests` Directory

Create a `tests` directory at the root level of your project if it doesn't exist. Rust automatically recognizes this as a directory where integration tests live.

```
mkdir tests
```

### 2. Add Your Test File

Inside the `tests` directory, create a new `.rs` file. Let's name it `tests_integration.rs` for this example.

```
touch tests/tests_integration.rs
```

### 3. Write Your Tests

Add your test functions inside `tests_integration.rs`. Use the `#[test]` attribute to denote a test function. 

```rust
// tests/tests_integration.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

### 4. Run Your Tests

To run all tests, navigate to the root directory of your project and execute:

```
cargo test
```

To run only the integration tests, execute:

```
cargo test --test tests_integration
```

### Troubleshooting

1. **Tests Aren't Recognized**: Make sure you placed your test files in the correct `tests` directory and your test functions are annotated with `#[test]`.

2. **Module or Import Errors**: If your test requires importing a module or using some items from your main application, you might have to adjust the import paths or add dependencies explicitly.

## Conclusion

Integration testing is an important aspect of Rust development for ensuring that different parts of your application work well together. With this basic tutorial, you should be able to set up and run simple integration tests in a Rust project.