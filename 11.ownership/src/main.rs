fn main() {
    // let s1 = String::from("abc");
    // let s2 = s1;

    // println!("{}", s1); // Error because we moved the value from s1 to s2, and not copied it

    let mut s1 = String::from("abc");
    let s2 = s1.clone();

    s1 = String::from("abcd");

    println!("{}, {}", s1, s2);
}
