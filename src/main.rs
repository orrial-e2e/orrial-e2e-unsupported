struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.increment();
    println!("count = {}", counter.value);
}
