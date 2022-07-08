use clap::Parser;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Args {
    #[clap(short = 'a', help = "Creates an array of words", parse(from_flag))]
    array: bool,

    #[clap(short = 'p', help = "Pretty-prints", parse(from_flag))]
    pretty: bool,

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

fn parse(args: Args) -> String {
    if args.array {
        do_array(args)
    } else {
        do_object(args)
    }
}

fn to_string<T: serde::ser::Serialize>(pretty: bool, value: T) -> String {
    if pretty {
        serde_json::to_string_pretty(&value).unwrap()
    } else {
        serde_json::to_string(&value).unwrap()
    }
}

fn do_object(args: Args) -> String {
    let mut obj = HashMap::new();
    for el in &args.values {
        let kv: Vec<&str> = el.split('=').collect();
        if kv.len() != 2 {
            panic!("'{}' must be key=value", el);
        }
        obj.insert(kv[0], parse_value(kv[1]));
    }
    to_string(args.pretty, &obj)
}

fn do_array(args: Args) -> String {
    let mut array: Vec<serde_json::Value> = Vec::new();
    for el in &args.values {
        array.push(parse_value(&el));
    }
    to_string(args.pretty, &array)
}

fn main() {
    let args = Args::parse();
    println!("{}", parse(args));
}
