use anyhow::{Context, Ok, Result};
use clap::Parser;
use serde_json::Value;
use std::{collections::HashMap, vec};
mod args;

fn main() -> Result<()> {
    let args = args::Args::parse();
    let border_type = args.border_type.to_ascii_lowercase();

    let border = match &border_type[..] {
        "none" => Some(comfy_table::presets::NOTHING),
        "utf8" => Some(comfy_table::presets::UTF8_FULL),
        "ascii" => Some(comfy_table::presets::ASCII_FULL),
        _ => None,
    };
    if border.is_none() {
        return Err(anyhow::Error::msg(format!(
            "Invalid border type \"{border_type}\""
        )));
    }

    let stdin = std::io::stdin();
    let content = std::io::read_to_string(stdin)?;

    // println!("{:?}", content);

    let root: Value = serde_json::from_str(&content).with_context(|| "Failed to parse JSON")?;

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
        return Err(anyhow::Error::msg("Input needs to be an array"));
    }

    let mut common_fields: HashMap<String, usize> = HashMap::new();

    // Get common fields
    for o in &objs {
        let o = o.as_object().unwrap();
        for k in o.keys() {
            common_fields.insert(k.clone(), common_fields.get(k).unwrap_or(&0) + 1);
        }
    }

    // Generate table

    let mut t = comfy_table::Table::new();
    let mut header: Vec<String> = vec![];

    // Header fields (keys)
    for (k, v) in &common_fields {
        if *v == objs.len() {
            header.push(k.clone());
        }
    }

    t.set_header(header.clone());
    t.load_preset(border.unwrap());

    // Value fields
    for o in objs {
        let mut row: Vec<String> = vec![];
        for k in &header {
            row.push(format!("{}", o.get(k).unwrap()));
        }
        t.add_row(row);
    }

    println!("{t}");
    Ok(())
}
