# input_loop

A utility for getting typed input from stdin with validation.

## Usage

Run this command in your terminal
```bash
cargo add input_loop
```

Then add this to your `Cargo.toml`:

```toml
[dependencies]
input_loop = version = "0.1.2"
```

## Example

```rust
use input_loop::input_loop;

fn main() {
    let number: i32 = input_loop("Enter a number: ");
    println!("You entered: {}", number);
}
```