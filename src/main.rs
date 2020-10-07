use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum OperationType {
    JsonToShape,
}

impl FromStr for OperationType {
    type Err = ();

    fn from_str(input: &str) -> Result<OperationType, Self::Err> {
        match input {
            "jsontoshape" => Ok(OperationType::JsonToShape),
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

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let operation_type = OperationType::from_str(&args[1]).unwrap();

    let filename = &args[2];

    let mut filepath = None;

    if args.len() == 4 {
        filepath = Some(&args[3]);
    }

    match operation_type {
        OperationType::JsonToShape => {
            if filepath == None {
                // cargo run jsontoshape ./testing.json
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = json_to_shape_process(contents);
                println!("{}", result);
            } else {
                // cargo run jsontoshape ./testing.json ./result.php
                let contents =
                    fs::read_to_string(filename).expect("Something went wrong when reading file");
                let result = json_to_shape_process(contents);
                fs::write(filepath.unwrap(), result).expect("Unable to write file.");
            }
        }
    }
}
