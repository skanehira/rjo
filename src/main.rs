use clap::Parser;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Arugs {
    #[clap(
        short = 'a',
        long = "array",
        help = "Creates an array of words",
        parse(from_flag)
    )]
    array: bool,
    #[clap()]
    values: Vec<String>,
}

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

fn parse(args: Arugs) -> String {
    if args.array {
        do_array(args)
    } else {
        do_object(args)
    }
}

fn do_object(args: Arugs) -> String {
    let mut obj = HashMap::new();
    for el in &args.values {
        let kv: Vec<&str> = el.split('=').collect();
        if kv.len() != 2 {
            panic!("'{}' must be key=value", el);
        }
        obj.insert(kv[0], parse_value(kv[1]));
    }
    serde_json::to_string(&obj).unwrap()
}

fn do_array(args: Arugs) -> String {
    let mut array: Vec<serde_json::Value> = Vec::new();
    for el in &args.values {
        array.push(parse_value(&el));
    }
    serde_json::to_string(&array).unwrap()
}

fn main() {
    let args = Arugs::parse();
    println!("{}", parse(args));
}
