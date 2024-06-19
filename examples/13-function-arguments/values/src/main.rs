
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    use std::io::{stdin, stdout, Write};
    let mut sinput = String::new();
    print!("Enter a list of numbers separated by spaces: ");
    let _ = stdout().flush();
    stdin().read_line(&mut sinput).expect("Failed to read line");
    let numbers: Vec<i32> = sinput.split_whitespace() // Split the input string into a vector of words
        .map(|s| s.parse().expect("parse error")) // Convert each word into a number
        .collect(); // Collect the numbers into a vector
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
