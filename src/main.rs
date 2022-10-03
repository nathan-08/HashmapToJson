use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use json;

fn main() {
    write(HashMap::<&'static str, usize>::from([
               ("one"  ,1),
               ("two"  ,2),
               ("three",3),
               ("four" ,4),
               ("five" ,5),
    ]));
    let map = read();
    for (key,value) in map {
        println!("{key}: {value}");
    }
}

fn write(map: HashMap<&'static str, usize>) {
    let mut object = json::JsonValue::new_object();
    for (key,value) in map {
        object[key] = value.into();
    }

    let stringified = object.pretty(5);
    let mut file = File::create("symbol_table.json")
        .expect("Failed to create file");
    write!(file, "{}", stringified)
        .expect("Failed to write to file");
}

fn read() -> HashMap<String, usize> {
    let mut map = HashMap::<String, usize>::new();
    let mut file = File::open("symbol_table.json")
            .expect("Failed to open file");
    let mut stringified = String::new();
    file.read_to_string(&mut stringified)
            .expect("Failed to read file");
    let parsed = json::parse(&stringified)
            .expect("Failed to parse json");
    for entry in parsed.entries() {
        let key = entry.0.to_owned();
        let value: f64 = match entry.1 {
            json::JsonValue::Number(n) => (*n).into(),
            _ => panic!("NaN")
        };
        map.insert(key, value as usize);
    }
    map
}
