fn main() {
    let msg = "Hello 🗺️"; // &str not modifiable

    let msg2 = String::from("Hello 🗺️"); // String modifiable

    // Can't index a string
    let character = msg[2]; // ERROR
}
