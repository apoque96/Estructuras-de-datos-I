use serde_json::{Result, Value};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn appartment(app: &Value, wanna_pet_friendly: bool, budget: f64){
    let mut ans = Vec::new();
    let app = app.as_array();
    if app.is_none(){
        print!("[]");
        return;
    }
    let app = app.unwrap();
    print!("[");
    for apartment in app{
        let pet_friendly = apartment["isPetFriendly"].as_bool();
        if pet_friendly.is_none(){
            continue;
        }
        let pet_friendly = pet_friendly.unwrap();

        let price = apartment["price"].as_f64();
        if price.is_none(){
            continue;
        }
        let price = price.unwrap();

        if price < budget{
            if (wanna_pet_friendly == pet_friendly) || wanna_pet_friendly == false{
                // let id = apartment["id"].as_str();
                // if id.is_none(){
                //     continue;
                // }
                // let id = id.unwrap();
                ans.push(apartment);
            }
        }
    }
    //Gracias Chatgpt, xd
    ans.sort_by(|a, b| {
        // Access the 'price' key from each map
        let price_a = a["price"].as_f64().unwrap_or(0.0); // default to 0.0 if not present or not a float
        let price_b = b["price"].as_f64().unwrap_or(0.0); // default to 0.0 if not present or not a float

        // Compare the prices
        price_a.partial_cmp(&price_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    for result in ans{
        let id = result["id"].as_str();
        if id.is_none(){
            continue;
        }
        let id = id.unwrap();
        print!("{}, ", id);
    }
}

// Lee el archivo linea por linea
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//Convierte la linea en el mapa de apartamentos
fn get_map(ln: &String) -> Result<Value> {
    let v: Value = serde_json::from_str(ln)?;

    Ok(v)
}

fn lab2(path: &str){
    //Lee el json
    let lines = lines_from_file(path);
    for ln in lines{
        let map = get_map(&ln);
        //Valida que se halla creado el mapa correctamente
        if  !map.is_ok() {
            println!("Couldn't read line");
            break;
        }
        let map = map.ok().unwrap();
        if map["input1"].is_null() || map["input2"].is_null(){
            println!("Incorrect file format");
            return;
        }
        // println!("{:?}", map["input1"][0]["builds"]["Houses"][0]["address"]);
        let building_type = map["input2"]["typeBuilder"].as_str();

        if !building_type.is_some(){
            println!("Couldn't read line");
            break;
        }
        let building_type = building_type.unwrap();
        match building_type{
            "Apartments" => {
                let district = map["input1"].as_array();
                if district.is_none(){
                    continue;
                }
                let district = district.unwrap();
                
                let wanna_pet_friendly = map["input2"]["wannaPetFriendly"].as_bool();
                if wanna_pet_friendly.is_none() {
                    continue;
                }
                let wanna_pet_friendly = wanna_pet_friendly.unwrap();

                let budget = map["input2"]["budget"].as_f64();
                if budget.is_none(){
                    continue;
                }
                let budget = budget.unwrap();
                for x in district{
                    appartment(&x["builds"]["Apartments"], wanna_pet_friendly, budget);
                }
            },
            _ => continue,
        }
    }
}
fn main() {
    lab2("./input/input_lab_2_example(2).jsonl");
}
