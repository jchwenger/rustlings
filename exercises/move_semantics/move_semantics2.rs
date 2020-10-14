// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {

    // 1

    println!("----------------------------------------");
    println!("1)");

    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // --------
    // 2

    println!("----------------------------------------");
    println!("2)");

    let vec2 = Vec::new();

    let mut vec3 = fill_vec_borrow(&vec2);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);

    vec3.push(88);

    println!("{} has length {} content `{:?}`", "vec3", vec3.len(), vec3);

    // --------
    // 3

    println!("----------------------------------------");
    println!("3)");

    let mut vec4 = Vec::new();

    fill_vec_mut_borrow(&mut vec4);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec4", vec4.len(), vec4);

}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn fill_vec_mut_borrow(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}


