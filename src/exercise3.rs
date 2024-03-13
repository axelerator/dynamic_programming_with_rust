use crate::read::get_i64;

fn prefill_vector() -> Vec<i64> {
    let mut v = Vec::with_capacity(92);
    v.push(0);
    v.push(1);
    for i in 2..v.capacity() {
        v.push(v[i - 2] + v[i - 1]);
    }
    v
}

pub fn main() {
    // Initialize the prefilled vector.
    let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    // let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // Calculate the Fibonacci number.
        println!("Prefilled:  {}", prefilled_values[n as usize]);
        // println!("On the fly: {}", fibonacci_on_the_fly(&mut fill_on_the_fly_values, n));
        // println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}
