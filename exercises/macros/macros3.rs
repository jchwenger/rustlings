// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// https://doc.rust-lang.org/rust-by-example/macros/designators.html

#[macro_use]
mod macros {
    macro_rules! my_macro {
       ($s: expr) => {
            println!("Check out my {} macro!", $s);
        };
    }
}

fn main() {
    my_macro!("cool".to_string());
}
