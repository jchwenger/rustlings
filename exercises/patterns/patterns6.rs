// patterns6.rs
// Make me compile! Execute `rustlings hint patterns6` for hints

fn is_smaller_than_five(num: i8) -> bool {
    match num {
        x if x < 5 => true,
        _ => false,
    }
}

fn main() {
    let num = 4;
    assert!(is_smaller_than_five(num));
    println!("Yipee! {} is lower than 5.", num);

}

