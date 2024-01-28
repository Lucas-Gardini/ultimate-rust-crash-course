// This throws and error, because we cannot modify a value of a "reference"
// fn main() {
//     let mut s1 = String::from("abc");
//     do_stuff(&s1);
//     println!("{}", s1);
// }

// fn do_stuff(s: &String) {
//     s = &String::from("12");
// }

// The correct way would be:

fn main() {
    let mut s1 = String::from("abc");
    do_stuff(&mut s1);
    println!("{}", s1);
}

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi, ");

    // or
    // *s = String::from("Replaced")
}
