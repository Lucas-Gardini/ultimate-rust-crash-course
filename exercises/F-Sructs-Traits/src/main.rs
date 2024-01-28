trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    left: u32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.left -= 1;
    }
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    for num in 0..5 {
        item.bite();
    }
}
