fn main() {
    // Example 1
    let x = 5;

    {
        let y = 99;
        println!("x = {}, y = {}", x, y);
    }

    // Error! y is not in scope
    println!("x = {}, y = {}", x, y);

    // Example 2
    let j = 10;
    {
        let j = 20;
        println!("The value of j is {}", j); // Prints "The value of j is 20"
    }

    println!("The value of j is {}", j); // Prints "The value of j is 10"
}
