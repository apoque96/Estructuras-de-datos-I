use serde_json::{Result, Value};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn find_appartment(appartment: &Value, wanna_pet_friendly: bool, budget: f64) -> Vec<&Value>{
    let mut ans = Vec::new();
    let appartment = appartment.as_array();
    if appartment.is_none(){
        return ans;
    }
    let appartment = appartment.unwrap();
    for apartment in appartment{
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

        if price <= budget && wanna_pet_friendly == pet_friendly{
            ans.push(apartment);
        }
    }
    ans

}

fn find_houses(houses: &Value, color: String, budget: f64) -> Vec<&Value>{
    let mut ans = Vec::new();
    let houses = houses.as_array();
    if houses.is_none(){
        return ans;
    }
    let houses = houses.unwrap();
    for house in houses{
        let zone = house["zoneDangerous"].as_str();
        if zone.is_none(){
            continue;
        }
        let zone = zone.unwrap();

        let price = house["price"].as_f64();
        if price.is_none(){
            continue;
        }
        let price = price.unwrap();

        if price <= budget{
            match color.as_str() {
                "Red" => {
                    if zone == "Red"{
                        ans.push(house);         
                    }
                },
                "Orange" => {
                    if zone == "Red" || zone == "Orange"{
                        ans.push(house);
                    }
                },
                "Yellow" => {
                    if zone == "Red" || zone == "Orange" || zone == "Yellow"{
                        ans.push(house);
                    }
                },
                "Green" => {
                    if zone == "Red" || zone == "Orange" || zone == "Yellow" || zone == "Green"{
                        ans.push(house);
                    }
                }
                _ => continue,
            }
            // ans.push(house);
        }
    }
    ans
}

fn find_premise(premises: &Value, commercial_activity: String, budget: f64) -> Vec<&Value>{
    let mut ans = Vec::new();
    let premises = premises.as_array();
    if premises.is_none(){
        return ans;
    }
    let premises = premises.unwrap();
    for premise in premises{
        let activities = premise["commercialActivities"].as_array();
        if activities.is_none(){
            continue;
        }
        let activities = activities.unwrap();

        let price = premise["price"].as_f64();
        if price.is_none(){
            continue;
        }
        let price = price.unwrap();

        for activity in  activities{
            let activity = activity.as_str();
            if activity.is_none(){
                continue;
            }
            let activity = activity.unwrap();

            if price <= budget && activity == commercial_activity{
                ans.push(premise);
            }
        }
    }
    ans
}

// Lee el archivo linea por linea
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
        let district = map["input1"].as_array();
        if district.is_none(){
            continue;
        }
        let district = district.unwrap();
        
        
        let budget = map["input2"]["budget"].as_f64();
        if budget.is_none(){
            continue;
        }
        let budget = budget.unwrap();
        let mut ans = Vec::new();
        match building_type{
            "Apartments" => {
                let wanna_pet_friendly = map["input2"]["wannaPetFriendly"].as_bool();
                if wanna_pet_friendly.is_none() {
                    continue;
                }
                let wanna_pet_friendly = wanna_pet_friendly.unwrap();
                for x in district{
                    ans.extend(find_appartment(&x["builds"]["Apartments"], wanna_pet_friendly, budget));
                }
            },
            "Premises" => {
                let commercial_activity = map["input2"]["commercialActivity"].as_str();
                if commercial_activity.is_none(){
                    continue;
                }
                for x in district{
                    let commercial_activity = String::from(commercial_activity.unwrap());
                    ans.extend(find_premise(&x["builds"]["Premises"], commercial_activity, budget));
                }
            },
            "Houses" => {
                let color = map["input2"]["minDanger"].as_str();
                if color.is_none(){
                    continue;
                }
                for x in district{
                    let color = String::from(color.unwrap());
                    ans.extend(find_houses(&x["builds"]["Houses"], color, budget))
                }
            }
            _ => continue,
        }

        //Gracias Chatgpt, xd
        ans.sort_by(|a, b| {
            // Access the 'price' key from each map
            let price_a = a["price"].as_f64().unwrap_or(0.0); // default to 0.0 if not present or not a float
            let price_b = b["price"].as_f64().unwrap_or(0.0); // default to 0.0 if not present or not a float

            // Compare the prices
            price_a.partial_cmp(&price_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        print!("[");
        {
        let mut i = 0;
        while i < ans.len(){
            let id = ans[i]["id"].as_str();
            if id.is_none(){
                continue;
            }
            let id = id.unwrap();
            print!("\"{}\"", id);
            if i < ans.len()-1{
                print!(",");
            }
            i += 1;
        }
        }
        println!("]");
    }
}
fn main() {
    lab2("./input/input_challenge_lab_2(1).jsonl");
}
