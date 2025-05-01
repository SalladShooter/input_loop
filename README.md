# input_loop

A utility for getting typed input from stdin with validation.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
input_loop = { version = "0.1.0", path = "path/to/input_loop" }
```

## Example

```rust
use input_loop;

fn main() {
    let number: i32 = input_loop("Enter a number: ");
    println!("You entered: {}", number);
}
```