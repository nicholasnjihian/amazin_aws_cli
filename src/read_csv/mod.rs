//Our Imports

use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;

/// This introduces a type alias so that we can conveniently reference our
/// record type.
type Record = HashMap<String, String>;

///helper function to read csv credentials file
///and extract "Access Key" and "Secret access key"
///to avoid having it printed to stdin
pub fn read_csv<R>(f: R) -> Result<Vec<String>, Box<dyn Error>>
where
    R: BufRead,
{
    let mut result_vec = Vec::new();
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        //deserialize our record into a Record type via type coercion
        let record: Record = result?;
        let secret_access_key = match record.get(&"Secret access key".to_string()) {
            Some(r) => r.to_string(),
            None => panic!("No secret access key found in csv file"),
        };
        let aws_access_key = match record.get(&String::from("Access key ID")) {
            Some(r) => r.to_string(),
            None => panic!("No access key in csv file"),
        };
        result_vec.push(aws_access_key);
        result_vec.push(secret_access_key);
    }
    Ok(result_vec)
}
