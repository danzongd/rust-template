use rayon::prelude::*;

fn main() {
    let sum: u64 = (0..=10).into_par_iter().sum();
    println!("{}", sum);
}
