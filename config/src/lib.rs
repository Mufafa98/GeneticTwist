pub mod random {
    use std::sync::Mutex;
    static RANDOM_INSTANCE: Seed = Seed {
        value: Mutex::new(31415),
    };

    pub struct Seed {
        value: Mutex<u64>,
    }

    impl Seed {
        pub fn new() -> &'static Self {
            &RANDOM_INSTANCE
        }

        pub fn get_seed(&self) -> u64 {
            let mut value = self.value.lock().unwrap();
            let seed_to_return = *value;
            *value += 1;
            seed_to_return
        }
    }
}
