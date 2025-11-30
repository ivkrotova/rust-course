/// Module to store helper functions.
mod helpers {

    /// Public greeting function.
    pub fn greet() {
        // returns unit type () by default, which is similar to void in other languages
        println!("Hello, world!");
    }
}

fn main() {
    // run the greeting function from the helpers module
    helpers::greet();
}
