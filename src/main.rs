mod exercise1;
mod exercise2;
mod exercise3;
mod exercise4;
mod exercise5;

mod read;

fn main() {
    let exercise = 5;
    match exercise {
        1 => exercise1::main(),
        2 => exercise2::main(),
        3 => exercise3::main(),
        4 => exercise4::main(),
        5 => exercise5::main(),
        _ => panic!("exercise must be 1, 2, 3, 4, or 5"),
    }
}
