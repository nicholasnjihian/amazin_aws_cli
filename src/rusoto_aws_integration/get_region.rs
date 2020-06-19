//Imports
use rusoto_core::Region;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

///The function 'get_aws_region' parses the credentials file to extract the Region
///as a rusoto_core::Region type
///This is required because this application does not generate
///the file referred to by the environmental variable $AWS_CONFIG which is at ~/.aws/config
///that is expected by the multiple rusoto credentials provider
pub fn get_aws_region<P>(aws_credentials_path: P) -> Region
where
    P: AsRef<Path>,
{
    let mut v = Vec::new();
    let f = fs::File::open(aws_credentials_path)
        .expect("Error opening the credentials file in attempt to extract AWS region");
    let reader = io::BufReader::new(f);
    for line_result in reader.lines() {
        let mut line = line_result.expect("Error parsing the lines in credentials file");
        if line.contains("region") {
            let offset = match line.find('='){
                Some(index) => index,
                None => panic!("Parsing region fromthe config file has failed. Errors in file.Please restart the application to reset the config file."),
            };
            let _ = line.drain(0..(offset + 1)).collect::<String>();
            let line = line.trim().to_owned();
            v.push(line);
        }
    }
    v[0].parse::<Region>()
        .expect("The region value for aws could not be obtained")
}
