//Our Imports
use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use structopt::StructOpt;

use aws_cli_challenge::*;

fn main() {
    let options = Awscli::from_args();
    println!("{:?}", options);

    //We pattern match against the enum variants 'configure' or 'list' for the Awscli
    //enum and perform action depending which variant is chosen
    //See below:
    match &options {
        Awscli::Configure(v) => match &v.credentials_file {
            Some(f) => {
                let file: File = File::open(f).unwrap();
                let reader = BufReader::new(file);

                let credentials = match read_csv::read_csv(reader) {
                    Ok(result) => result,
                    Err(err) => panic!("Problem reading csv file: Err == {}", err),
                };
                let mut writer = match create_cred_config::create_config_file() {
                    Ok(w) => w,
                    Err(err) => panic!("Problem creating config file ==> Err: {}", err),
                };
                let _ = write_config::edit_config_from_csv(&credentials, &mut writer).unwrap();
                //we need to prompt user to give us details for region and also
                //write these to the hidden config file in the user's home filesystem
                //we made at ~/.aws_cli/credentials
                let _ = write_config::edit_config_from_stdin("Enter AWS_REGION => ", &mut writer)
                    .expect("Error editing AWS Region..");
            }
            None => {
                //We assume there is a file already ==> '~/.aws/credentials'
                //But we will need to check it exists and it is valid
                let _ = verify_config().unwrap();
            }
        },
        Awscli::List(l) => {
            unimplemented!();
        }
    }
}

fn verify_config() -> Result<(), Box<dyn Error>> {
    let credentials_path = Path::new("~/.aws_cli/credentials");
    if credentials_path.exists() && credentials_path.is_file() {
        let f = File::open(credentials_path)?;
        let buffer = BufReader::new(f);
        for line_result in buffer.lines() {
            let line = line_result?;
            if !line.contains("AWS_ACCESS_KEY_ID")
                || !line.contains("AWS_SECRET_ACCESS_KEY")
                || !line.contains("AWS_REGION")
            {
                eprintln!("The credentials file is invalid!");
                let mut writer = match create_cred_config::create_config_file() {
                    Ok(w) => w,
                    Err(err) => panic!("Problem creating config file ==> Err: {}", err),
                };
                let required = [
                    "AWS_ACCESS_KEY_IDi = ",
                    "AWS_SECRET_ACCESS_KEY = ",
                    "AWS_REGION = ",
                ];
                for item in &required {
                    let _ = write_config::edit_config_from_stdin(item, &mut writer)
                        .expect("Error editing credentials file for item ");
                }
            }
        }
    }
    Ok(())
}
