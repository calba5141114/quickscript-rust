use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum OperationType {
    JsonToShape,
    ShapeToJson,
}

impl FromStr for OperationType {
    type Err = ();

    fn from_str(input: &str) -> Result<OperationType, Self::Err> {
        match input {
            "jsontoshape" => Ok(OperationType::JsonToShape),
            "shapetojson" => Ok(OperationType::ShapeToJson),
            _ => Err(()),
        }
    }
}

/**
 * converts json to hack shape
 */
fn json_to_shape_process(contents: String) -> String {
    let double_quote_char = '"';
    let check_for_match = format!(": {}", double_quote_char);
    let second_for_match = ": shape(";
    return contents
        .replace("{", "shape(")
        .replace("}", ")")
        .replace("[", "vec[")
        .replace(&check_for_match, &format!(" => {}", double_quote_char))
        .replace(second_for_match, " => shape(");
}

fn json_to_shape_process_new(contents: String) -> String {
    const DOUBLE_QUOTE: char = '"';
    let mut process_store: Vec<String> = Vec::new();
    let character_vec: Vec<char> = contents.chars().collect();

    let check_for_match = format!(": {}", DOUBLE_QUOTE);
    let second_for_match = ": shape(";

    for node in character_vec {
        match node {
            '{' => process_store.push("shape(".to_string()),
            '}' => process_store.push(")".to_string()),
            '[' => process_store.push("vec[".to_string()),
            _ => process_store.push(node.to_string()),
        }
    }

    let process_store_str: String = process_store
        .join("")
        .replace(&check_for_match, &format!(" => {}", DOUBLE_QUOTE))
        .replace(second_for_match, " => shape(");
    return process_store_str;
}

/**
 * converts hack shape to JSON
 */
fn shape_to_json_process(contents: String) -> String {
    let double_quote_char = '"';
    let check_for_match = format!(": {}", double_quote_char);
    return contents
        .replace("shape(", "{")
        .replace(")", "}")
        .replace("vec[", "[")
        .replace(&format!(" => {}", double_quote_char), &check_for_match)
        .replace("=> {", ": {");
}

/**
 * Log Enviroment Arguments.
 */
#[cfg(debug_assertions)]
fn log_env_args() {
    let args: Vec<String> = env::args().collect();
    println!("Debug Enviroment Args: {:?}", args);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    log_env_args();

    let operation_type = OperationType::from_str(&args[1]).unwrap();

    let filename = &args[2];

    let mut filepath = None;

    if args.len() == 4 {
        filepath = Some(&args[3]);
    }

    match operation_type {
        OperationType::JsonToShape => {
            if filepath == None {
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = json_to_shape_process_new(contents);
                println!("{}", result);
            } else {
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = json_to_shape_process_new(contents);
                fs::write(filepath.unwrap(), result).expect("Unable to write file.");
            }
        }
        OperationType::ShapeToJson => {
            if filepath == None {
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = shape_to_json_process(contents);
                println!("{}", result);
            } else {
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = shape_to_json_process(contents);
                fs::write(filepath.unwrap(), result).expect("Unable to write file.");
            }
        }
    }
}
