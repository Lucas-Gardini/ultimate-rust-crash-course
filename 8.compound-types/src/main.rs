fn tuple() {
    let info = (1, 2, 3, 5.5); // Tuple

    let first = info.0;

    let (one, two, three) = info;
}

// Has a maximum size of 32 items
fn array() {
    let vuf = [1, 2, 3];
    let zeros = [0; 10]; // Becomes: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
}

fn main() {
    let info = (1, 2, 3, 5.5); // Tuple

    let first = info.0;

    let (one, two, three) = info;
}
