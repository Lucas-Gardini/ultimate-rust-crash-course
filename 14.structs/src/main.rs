struct RedFox {
    enemy: bool,
    life: u8,
}

trait Noisy {
    fn get_noise(&self) -> &str;
    fn get_name(&self) {
        println!("Noisy!");
    }
}

impl RedFox {
    fn newDefault() -> Self {
        Self {
            enemy: true,
            life: 100,
        }
    }

    fn new(enemy: bool, life: u8) -> Self {
        Self {
            enemy: enemy,
            life: life,
        }
    }
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

impl Noisy for bool {
    fn get_noise(&self) -> &str {
        "BULEAN"
    }
}

fn main() {
    let fox = RedFox::new(true, 70);
    let fox2 = RedFox::newDefault();

    print_noise(fox);
    print_noise(true);

    println!("{}", true.get_noise());
    true.get_name();
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}
