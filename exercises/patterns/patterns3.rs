// patterns3.rs
// Make me compile! Execute `rustlings hint patterns3` for hints

fn match_me(x: Option<i32>) -> Option<i32> {
    match x {
        Some(50) => Some(50),
        Some(y) => Some(y),
        _ => None
    }
}

fn main() {

    let x = Some(50);
    assert_eq!(match_me(x), Some(50));
    println!("Match_me is now: {:?}", match_me(x));

    let x = Some(8);
    assert_eq!(match_me(x), Some(8));
    println!("Match_me is now: {:?}", match_me(x));

    let x = None;
    assert_eq!(match_me(x), None);
    println!("Match_me is now: {:?}", match_me(x));


}

