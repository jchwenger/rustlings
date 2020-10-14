// patterns7.rs
// Make me compile! Execute `rustlings hint patterns7` for hints

enum WishList {
    Wish { id: i32 },
}

fn main() {

    let my_wish = WishList::Wish { id: 5 };

    match my_wish {
        WishList::Wish {
            id: id_variable @ 1..=5,
        } => println!("Found wish number: {}", id_variable),
        WishList::Wish { id: 5..=10 } => {
            println!("Found a wish between 5 and 10.")
        }
        WishList::Wish { id } => println!("Found some other wish: {}", id),
    }

}

