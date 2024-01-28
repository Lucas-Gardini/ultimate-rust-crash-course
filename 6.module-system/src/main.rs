use module_system::greet; // optional
use rand::thread_rng; // optional
use rand::Rng; // optional

fn main() {
    // We can call from the application "scope"
    module_system::greet();

    // Or with the use on the top
    greet();

    // Using a module (or crate) from the cargo.toml

    // With the crate scope
    let x = rand::thread_rng().gen_range(0, 100);

    // Or with the use on the top
    let y = thread_rng().gen_range(0, 100);

    println!("{}, {}", x, y);
}
