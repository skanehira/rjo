use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::slice::Iter;

fn usage() {
    println!(
        "{}",
        r#"Usage of rjo
examples:
  rjo name=gorilla age=10 # => {"name": "gorilla", "age": 10}
"#
    );
    exit(1);
}

fn parse_value(value: &str) -> serde_json::Value {
    match value {
        "true" => serde_json::Value::Bool(true),
        "false" => serde_json::Value::Bool(false),
        "null" => serde_json::Value::Null,
        _ => match value.chars().nth(0).unwrap() {
            '{' | '[' | '"' => serde_json::from_str(value).unwrap(),
            '0'..='9' => serde_json::Value::Number(value.parse().unwrap()),
            _ => serde_json::Value::String(value.to_string()),
        },
    }
}

fn do_object(args: Iter<String>) -> String {
    let mut obj = HashMap::new();
    for el in args {
        let kv: Vec<&str> = el.split("=").collect();
        if kv.len() != 2 {
            panic!("'{}' must be key=value", el);
        }
        obj.insert(kv[0], parse_value(kv[1]));
    }
    serde_json::to_string(&obj).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        usage();
    }

    println!("{}", do_object(args.iter()));
}
