use std::fs::File;

enum Color {
    red,
    blue,
    green,
}

enum ComplexEnum {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

enum GenericEnum<T> {
    Yes(T),
    No,
}

fn main() {
    let color = Color::red;

    let coordiantes = ComplexEnum::Place { x: 24, y: 48 };

    // Auto imported enum (Option) that implements Some and None
    let mut x = None;
    x = Some(10);
    x.is_some(); // true
    x.is_none(); // false

    // Auto imported enum (Result) that implmements Ok and Err

    let f = File::open("foo").unwrap(); // If it's and OK the program continues, if not, tje program crashes

    // Another way to do it is
    let f = File::open("foo").expect("Error when trying to open file"); // If it's and OK the program continues, if not, the program crashes with the text

    // Another wayt to do it is
    let f = File::Open("foo");

    match f {
        Ok(f) => { /* do something */ }
        Err(e) => { /* do something */ }
    }
}
