// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number = "3"; // don't change this line
    println!("Number {}", number);

    // // very nice but what this exo is about
    // let num2 = match number.parse::<u8>() {
    //     Ok(num) => num,
    //     Err(e) => return Err(e),
    // };
    // println!("Number {}", num2);

    let number = 3;
    println!("Number {}", number);
    Ok(())
}
