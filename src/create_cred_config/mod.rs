//Our Imports

use std::env;
use std::error::Error;
use std::fmt::Write;
use std::fs::{self, File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

///Here we will create the config file where credentials will be stored in disk
///to avoid prompting for the credentials or reading from csv.
pub fn create_config_file() -> Result<BufWriter<File>, Box<dyn Error>> {
    let mut cred_path = String::new();

    //we need to get the value of $HOME environment variable,
    //maybe its /home/satoshi, or /home/napoleon-bonaparte

    let home = env::var("HOME")?;

    //append to the home directory a hidden directory for our credentials
    //which will be in ~/.aws/credentials file.
    write!(cred_path, "{}/.aws_cli", home)?;
    println!("path we will store credentials{:?}", cred_path);
    let basepath = Path::new(&cred_path);
    if basepath.exists() == false {
        //Does ~/.aws_cli already exist? If not:
        fs::create_dir(basepath)?;
    }
    let fullpath = basepath.join("credentials"); //append filename to make full path
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) //if file already exists,
        //then delete everything by making file contents zero for new credentials
        .open(fullpath)?;

    Ok(BufWriter::new(file))
}
