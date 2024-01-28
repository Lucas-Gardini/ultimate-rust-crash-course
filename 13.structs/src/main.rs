struct RedFox {
    enemy: bool,
    life: u8,
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

fn main() {
    let fox = RedFox::new(true, 70);
    let fox2 = RedFox::newDefault();
}
