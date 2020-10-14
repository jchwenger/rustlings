// patterns5.rs
// Make me compile! Execute `rustlings hint patterns5` for hints

fn main() {
   let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

