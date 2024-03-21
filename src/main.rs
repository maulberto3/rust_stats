// Some examples about Traits in Rust
mod m1 {
    // bark_trait.rs

    pub trait Dog {
        fn bark(&self);
    }

    impl Dog for i32 {
        fn bark(&self) {
            println!("Bark! Bark! I'm an integer: {}", self);
        }
    }

    // main.rs

    // mod bark_trait; // Import the module containing the trait definition

    // use bark_trait::Dog; // Import the trait into scope

    pub fn run() {
        let number: i32 = 42;
        number.bark(); // Call the bark method on an integer
    }
}

mod m2 {
    // Adjusting to a simple generic

    // bark_trait.rs

    pub trait Dog {
        fn bark(&self);
    }

    impl<T: std::fmt::Display> Dog for T {
        fn bark(&self) {
            println!("Bark! Bark! I'm something: {}", self);
        }
    }

    // main.rs

    // mod bark_trait;

    // use bark_trait::Dog;

    pub fn run() {
        let number: i32 = 42;
        let word = "hello";

        number.bark(); // Call the bark method on an integer
        word.bark(); // Call the bark method on a string slice
    }
}

mod m3 {

    // bark_trait.rs

    pub trait Dog {
        fn bark(&self);
    }

    impl<T: std::fmt::Display> Dog for T {
        fn bark(&self) {
            println!("Bark! Bark! I'm something: {}", self);
        }
    }

    pub fn make_dog_bark<T: Dog>(dog: T) {
        dog.bark();
    }

    // main.rs

    // mod bark_trait;

    // use bark_trait::{Dog, make_dog_bark};

    pub fn run() {
        let number: i32 = 42;
        let word = "hello";

        make_dog_bark(number); // Call make_dog_bark with an integer
        make_dog_bark(word); // Call make_dog_bark with a string slice
    }
}

fn main() {
    println!("Just some examples about Traits");
    m1::run();
    m2::run();
    m3::run();
}
