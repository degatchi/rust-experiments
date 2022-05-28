use model::Hands;

mod model {

    // Traits are APIs for types that use the interface.
    use std::fmt::Display;

    #[allow(dead_code)]
    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }

    impl Display for Fruit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Fruit::Apple => f.write_str("an apple"),
                Fruit::Banana => f.write_str("a banana"),
                Fruit::Kiwi => f.write_str("a kiwi"),
            }
        }
    }

    pub struct Hands {
        left: Option<Fruit>,
        right: Option<Fruit>,
    }

    impl Hands {
        pub fn new() -> Self {
            Self {
                left: Some(Fruit::Apple),
                right: Some(Fruit::Banana),
            }
        }

        #[allow(clippy::manual_swap)]
        pub fn juggle(mut self) -> Self {
            println!("Lets juggle");
            let air = self.left;
            self.left = self.right;
            self.right = air;
            self
        }

        pub fn report(&self) {
            report_item(&self.left, "Left");
            report_item(&self.right, "Right");
        }
    }

    // Only available for values of T that implement the Displayable trait.
    pub fn report_item<T: Display>(item: &Option<T>, which: &str) {
        match item {
            Some(what) => {
                println!("{} hand is holding {}", which, what);
            }
            _ => {
                println!("{} hand isn't holding anything", which);
            }
        }
    }
}

#[allow(dead_code)]
pub fn example_1() {
    let mut hands = Hands::new();
    hands.report();
    hands = hands.juggle();
    hands.report();
}
