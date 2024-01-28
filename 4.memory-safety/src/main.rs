fn main() {
    let enigma: i32;
    println!("enigma: {}", enigma); // error: use of possibly uninitialized variable: `enigma`
}

// Explanation: Rust does not allow the use of uninitialized variables.
