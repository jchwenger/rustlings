// patterns4.rs
// Make me compile! Execute `rustlings hint patterns4` for hints

fn main() {

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            assert_eq!(first, 2);
            assert_eq!(third, 8);
            assert_eq!(fifth, 32);
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

}
