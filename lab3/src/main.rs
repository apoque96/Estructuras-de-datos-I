use crate::auction::get_winner;

pub mod heap;
pub mod customers;
pub mod auction;

fn main() {
    get_winner();
}
