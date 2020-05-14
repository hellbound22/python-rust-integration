use std::slice;
use std::io::Cursor;

use serde::{Deserialize, Serialize};

use num_format::{Locale, ToFormattedString};

use calamine::{Reader, Xlsx, RangeDeserializerBuilder};

mod tests;

#[derive(Serialize, Deserialize)]
struct Product {
    sku: String,
    designation: String,
    price: i32,
    description: String,
}

impl Product {
    pub fn new_from_tuple(r: (String, String, i32)) -> Product {
        let (sku, designation, price) = r;

        let description = format!("{} ({}), ${}", designation, sku, price.to_formatted_string(&Locale::en));
        Product{sku, designation, price, description}
    }
}

pub fn _parse(data: &[u8]) -> Result<String, String> {
    let mut arr: Vec<Product> = Vec::new();

    let mut doc = match Xlsx::new(Cursor::new(data)) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(sheet) = doc.worksheet_range("Sheet1") {
        if let Ok(range) = sheet {
            let iter = RangeDeserializerBuilder::new().has_headers(false).from_range(&range).unwrap();
            
            for e in iter {
                let x = Product::new_from_tuple(e.unwrap());
                arr.push(x);
            }

            Ok(serde_json::to_string(&arr).unwrap())

        } else {
            return Err("No worksheet named 'Sheet1'".to_owned());
        }
    } else {
        return Err("Error specific to the file type".to_owned());
    }
}

pub fn write_to_memory(mem: &mut [u8], s: &str) {
    for (i, c) in s.as_bytes().to_vec().iter().enumerate() {
        mem[i] = *c;
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse(data: *const u8, len: usize, out_d: *mut u8, out_len: usize) -> i8 {
    let input = slice::from_raw_parts(data, len);
    let output = slice::from_raw_parts_mut(out_d, out_len);

    let ret =  match _parse(input) {
        Ok(s) => s,
        Err(e) => {
            write_to_memory(output, &e);
            return -1;
        }
    };

    write_to_memory(output, &ret);

    0
}
