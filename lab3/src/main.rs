use crate::auction::get_winner;

pub mod auction;
pub mod customers;
pub mod sort;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    get_winner();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
