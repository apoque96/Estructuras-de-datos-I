use std::vec;

use crate::heap::create_heap;
use crate::customers::create_customer_of_property;

pub mod heap;
pub mod customers;
fn main() {
    let arr = vec![
        create_customer_of_property(9367494915111, 6093, "2023-04-17T00:43:17.258Z"),
        create_customer_of_property(4966838245729, 7185, "2023-04-17T00:43:17.258Z"),
        create_customer_of_property(2065484727973, 7790, "2023-04-17T00:43:17.258Z"),
        create_customer_of_property(5461536416249, 5550, "2023-04-17T00:43:17.258Z")];

    let mut arr = create_heap(arr);

    loop {
        let element = arr.pop();
        match element {
            Some(x) => { println!("{:?}", x);},
            None => { break; },
        }
    }
}
