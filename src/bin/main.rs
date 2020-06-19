//Our Imports
//use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use structopt::StructOpt;

use aws_cli_challenge::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
            let config_location = create_cred_config::return_config_location()?;
            let reg = rusoto_aws_integration::get_region::get_aws_region(config_location.clone());
            match &l.resource {
                Resource::S3 => {
                    rusoto_aws_integration::list_s3_bucket(config_location, reg).await?
                }
                Resource::EC2 => rusoto_aws_integration::describe_ec2(config_location, reg).await,
                Resource::ECS => {
                    rusoto_aws_integration::list_ecs_container_clusters(config_location, reg).await
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn verify_config() -> Result<(), Box<dyn Error>> {
    let credentials_path = Path::new("~/.aws_cli/credentials");
    if credentials_path.exists() && credentials_path.is_file() {
        let f = File::open(credentials_path)?;
        let buffer = BufReader::new(f);
        for line_result in buffer.lines() {
            let line = line_result?;
            if !line.contains("aws_access_key_id=")
                || !line.contains("aws_secret_access_key=")
                || !line.contains("region=")
            {
                eprintln!("The credentials file is invalid!");
                let mut writer = match create_cred_config::create_config_file() {
                    Ok(w) => w,
                    Err(err) => panic!("Problem creating config file ==> Err: {}", err),
                };
                let required = ["aws_access_key_id=", "aws_secret_access_key=", "region="];
                for item in &required {
                    let _ = write_config::edit_config_from_stdin(item, &mut writer)
                        .expect("Error editing credentials file for item ");
                }
            }
        }
    }
    Ok(())
}
