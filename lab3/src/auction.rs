use serde_json::{Result, Value};
use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};
use std::collections::HashMap;

use crate::customers::Customer;
use crate::customers::create_customer;

use crate::customers::Winner;
// use crate::customers::create_winner;

// Reads the file line by line
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//Convierte la linea en el mapa
fn get_map(ln: &String) -> Result<Value> {
    let v: Value = serde_json::from_str(ln)?;

    Ok(v)
}
fn load_db(customers_db_strings: Vec<String>) -> HashMap<u64, Customer>{
    let mut customers_db = HashMap::new();
    // get all data from the customers and then creates
    // a struct of each customer
    for customer in customers_db_strings{
        let map = get_map(&customer);
        if  !map.is_ok() {
            println!("Couldn't read line");
            break;
        }
        let map = map.ok().unwrap();
        // Just some error handling LOL
        // I don't know why it's being dropped
        let dpi = match map["dpi"].as_u64() {
            Some(x) => { x },
            None => {break;},
        };
        let first_name = match map["firstName"].as_str() {
            Some(x) => { x.to_string() },
            None => {break;},
        };
        let last_name = match map["lastName"].as_str() {
            Some(x) => { x.to_string() },
            None => {break;},
        };
        let birth_date = match map["birthDate"].as_str() {
            Some(x) => { x.to_string() },
            None => {break;},
        };
        let job = match map["job"].as_str() {
            Some(x) => { x.to_string() },
            None => {break;},
        };
        let place_job = match map["placeJob"].as_str() {
            Some(x) => { x.to_string() },
            None => {break;},
        };
        let salary = match map["salary"].as_u64() {
            Some(x) => { x },
            None => {break;},
        };
        let element = create_customer(
            dpi, 
            first_name, 
            last_name, 
            birth_date, 
            job, 
            place_job, 
            salary);
        customers_db.insert(dpi, element);
        println!("{:?}", customers_db.get(&dpi));
    }
    customers_db
}

pub fn get_winner(){
    let customers_db = load_db(lines_from_file("./input/input_customer_example_lab_3(1).jsonl"));
    let auctions = lines_from_file("./input/input_auctions_example_lab_3(1).jsonl");
    determine_winner(&auctions[0]);
}

fn determine_winner(auction: &String) -> Option<Winner>{
    let auction = get_map(auction);

    if !auction.is_ok(){
        return None;
    }

    // create_winner(dpi, first_name, last_name, birth_date, job, place_job, salary, property, budget)
    None
}