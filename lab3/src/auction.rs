use serde_json::{Result, Value};
use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};
use std::collections::HashMap;

use crate::customers::Customer;
use crate::customers::create_customer;

use crate::customers::Winner;
use crate::customers::create_winner;

use crate::heap::create_heap;
use crate::customers::create_customer_of_property;
use crate::customers::CustomerOfProperty;

//Hashes the dpi
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn hash_u64(value: u64) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hashed_value = hasher.finish();
    format!("{:x}", hashed_value)
}

// Reads the file line by line
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// Converts the line into a hashmap
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
        // Reminder: avoid lifetimes unless I know what the hell they do
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
    }
    customers_db
}

fn determine_winner(auction: &String, customers_db: &HashMap<u64, Customer>) -> Option<Winner>{
    let auction = get_map(auction);
    let auction = auction.unwrap();

    let property = auction["property"].as_str()?;
    let mut rejection = auction["rejection"].as_u64()?;
    let customers: Vec<CustomerOfProperty> = auction["customers"].as_array()?
        .to_vec().iter().map( |value| {
            create_customer_of_property(
                value["dpi"].as_u64().unwrap(),
                value["budget"].as_u64().unwrap(),
                value["date"].as_str().unwrap().to_string(),
            )
        }).collect();

    let mut max_heap = create_heap(customers);
    
    while rejection > 0{
        max_heap.pop();
        rejection -= 1;
    }

    let winner = max_heap.pop()?;
    let customer = customers_db.get(&winner.dpi)?;

    Some(create_winner(
            winner.dpi, 
            winner.date.clone(),
            customer.first_name.clone(), 
            customer.last_name.clone(), 
            customer.birth_date.clone(), 
            customer.job.clone(), 
            customer.place_job.clone(), 
            customer.salary, 
            property.to_string(), 
            winner.budget,
            hash_u64(winner.dpi)))
}

pub fn get_winner(){
    let customers_db = load_db(lines_from_file("./input/input_customer_challenge_lab_3.jsonl"));
    let auctions = lines_from_file("./input/input_auctions_challenge_lab_3.jsonl");
    for auction in auctions{
        let winner = determine_winner( &auction, &customers_db);
        if !winner.is_some(){
            println!("no winner");
            return;
        }
        let winner = winner.unwrap();

        let json = serde_json::to_string(&winner).unwrap();
        println!("{}", json);
    }
}
