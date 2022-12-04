use std::{collections::HashMap, vec};

use serde_json::Value;

fn main() {
    let stdin = std::io::stdin();
    let content = std::io::read_to_string(stdin).unwrap();

    // println!("{:?}", content);

    let root: Value = serde_json::from_str(&content).unwrap();

    let mut objs: Vec<Value> = vec![];

    // Get all nested objects
    if root.is_array() {
        let array = root.as_array().unwrap();
        for o in array {
            if o.is_object() {
                objs.push(o.clone());
            }
        }
    } else {
        println!("Input needs to be an array");
        std::process::exit(1);
    }

    let mut common_fields: HashMap<String, usize> = HashMap::new();

    // Get common fields
    for o in &objs {
        let o = o.as_object().unwrap();
        for k in o.keys() {
            common_fields.insert(k.clone(), common_fields.get(k).unwrap_or(&0) + 1);
        }
    }

    let mut t = comfy_table::Table::new();
    let mut header: Vec<String> = vec![];

    for (k, v) in common_fields.iter() {
        if *v == objs.len() {
            header.push(k.clone());
        }
    }

    t.set_header(header.clone());
    t.load_preset(comfy_table::presets::NOTHING);

    for o in objs {
        let mut row: Vec<String> = vec![];
        for k in &header {
            row.push(format!("{}", o.get(k).unwrap()));
        }
        t.add_row(row);
    }

    println!("{}", t);
}
