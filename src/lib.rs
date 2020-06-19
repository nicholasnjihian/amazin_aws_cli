//Our Imports
use std::path::PathBuf;
use structopt::StructOpt;

//Separate our functions into separate namespaces(files)/modules for readability
//and to maintain code easily
pub mod create_cred_config;
pub mod read_csv;
pub mod rusoto_aws_integration;
pub mod write_config;

//Below are the structs and enums that will have StructOpt implementations
//to enable our cli program function.
//StructOpt is a Rust crate that allows us to create CLIs easily
//using a declarative style that incorporates macros(in few words).
//i.e. the derive macro.The macros generate code for us including a help option and statement.
#[derive(StructOpt, Debug)]
///"The various AWS resources we want to query"
pub enum Resource {
    S3,  //Amazin Simple Storage Service
    EC2, //Elastic Compute Cloud
    IAM, // Identity and Access Management
    RDS, //Relational Database Service
    ECR,
    ECS, //Elastic Container Service
}

#[derive(StructOpt, Debug)]
///"This subcommand will perform the work of listing the resource e.g 'ls s3'"
//doc comment above similar to --> '#[structopt(about = "--- This subcommand will perform the work of listing the resource e.g 'ls s3'")]'
pub struct List {
    #[structopt(subcommand)]
    pub resource: Resource,
}

/// "configure looks for a file in a default location that is $HOME/.awscli,
/// otherwise it will read from the csv file you download from AWS
/// when you create an IAM user and/or group which has
/// the AWS access key, secret key and region. Hence, it takes in file(-f or --file) which is optional parameter."
#[derive(StructOpt, Debug)]
pub struct Configure {
    ///"Specify the file
    ///i.e. the credentials file to read from"
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub credentials_file: Option<PathBuf>,
}

//The base command of our cli program will be defined by this struct (Awscli)
//i.e 'awscli <subcommmand/arguments/flags> <options>'
///takes subcommands 'configure' or 'ls'
///to read credentials from a credentials_file
///or it lists the AWS resources we want to view/query respectively.
#[derive(StructOpt, Debug)]
pub enum Awscli {
    Configure(Configure),
    #[structopt(alias = "ls")]
    List(List),
}
