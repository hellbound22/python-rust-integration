use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn description_formating() {
   if let Ok(js) = crate::_parse(&get_file()) {
       if let Ok(v) = serde_json::from_str::<Value>(&js) {
            assert_eq!(v[0]["description"], r#"Mac Book Pro 13” (MP13-0001), $1,299"#);
       }
       
   }
}

#[test]
fn parsing() {
    assert!(crate::_parse(&get_file()).is_ok())
}

fn get_file() -> Vec<u8> {
    let mut file = File::open("tests/input.xlsx").unwrap();
    let mut s: Vec<u8> = Vec::new();

    file.read_to_end(&mut s).unwrap();
    s
}
