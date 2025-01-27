# Rust Mutable Borrowing Example

This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same variable. Rust's ownership system prevents data races by disallowing this scenario. The example shows how mutable borrowing works and the compiler error that arises when multiple mutable references are attempted.

## Running the code

1. Clone this repository.
2. Navigate to the directory in your terminal.
3. Run `rustc bug.rs` and then `./bug` to compile and run the example. Uncomment the lines to see the error.