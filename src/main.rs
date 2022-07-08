use clap::{arg, parser::ValuesRef, Command};
use std::collections::HashMap;

fn parse_value(value: &str) -> serde_json::Value {
    match value {
        "true" => serde_json::Value::Bool(true),
        "false" => serde_json::Value::Bool(false),
        "null" => serde_json::Value::Null,
        _ => match value.chars().next().unwrap() {
            '{' | '[' | '"' => serde_json::from_str(value).unwrap(),
            '0'..='9' => serde_json::Value::Number(value.parse().unwrap()),
            _ => serde_json::Value::String(value.to_string()),
        },
    }
}

fn do_object(args: ValuesRef<String>) -> String {
    let mut obj = HashMap::new();
    for el in args {
        let kv: Vec<&str> = el.split('=').collect();
        if kv.len() != 2 {
            panic!("'{}' must be key=value", el);
        }
        obj.insert(kv[0], parse_value(kv[1]));
    }
    serde_json::to_string(&obj).unwrap()
}

fn do_array(args: ValuesRef<String>) -> String {
    let mut array: Vec<serde_json::Value> = Vec::new();
    for el in args {
        array.push(parse_value(el));
    }
    serde_json::to_string(&array).unwrap()
}

fn main() {
    let matches = Command::new("rjo")
        .name("rjo")
        .version("0.0.1")
        .author("skanehira")
        .about("This is jpmens/jo ported with Rust")
        .args(&[
            arg!(-a --array "creates an array of words").required(false),
            arg!(<VALUE>).multiple(true),
        ])
        .get_matches();

    let values = matches.get_many::<String>("VALUE").unwrap();

    let result = if matches.contains_id("array") {
        do_array(values)
    } else {
        do_object(values)
    };
    println!("{}", result);
}
