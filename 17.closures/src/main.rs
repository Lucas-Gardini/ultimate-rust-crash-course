fn main() {
    let add = |x, y| x + y;

    add(1, 2); // returns 3

    // using closures on a vector
    let mut v = vec![2, 4, 6];

    // Creates a iterator for the vector
    v.iter()
        // Map the vector and do something with it's values (the sintax is similar to a arrow function)
        .map(|x| x * 3)
        // Then filter it using another closure
        .filter(|x| *x > 10);
}
