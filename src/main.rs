mod lib;
use lib::input_loop;

fn main() {
    // Example usage
    let number: i32 = input_loop("Enter a number: ");
    println!("You entered: {}", number);
    let float: f64 = input_loop("Enter a decimal: ");
    println!("You entered: {}", float);
}