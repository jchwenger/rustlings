// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// https://stackoverflow.com/a/43080280
// https://stackoverflow.com/a/24159933
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    let other = current_favorite_static_color();
    println!("My current favorite static color is {}", other);

    let sth = String::from("blah!");
    println!("Something else I'd like to say: {}.", sth);

    let s: &str = &sth[..]; // full slice of sth
    println!("Almost the same: {}.", s);

    let s_moa: &str = &*sth; // full slice of sth
    println!("Yet again: {}.", s_moa);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}

fn current_favorite_static_color() -> &'static str {
    "green"
}
