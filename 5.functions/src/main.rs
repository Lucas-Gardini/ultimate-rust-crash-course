fn main() {
    println!("{}", do_stuff("Hello".to_string(), 32));
    println!("{}", do_more_stuff("Hi".to_string(), 12));
}

fn do_stuff(text: String, number: i32) -> String {
    return text + &number.to_string();
}

fn do_more_stuff(text: String, number: i32) -> String {
    // No need to use return keyword! (see that there is no semicolon at the end of the line)
    text + &number.to_string()
}
