[![Build Status](https://travis-ci.com/nicholasnjihian/amazin_aws_cli.svg?branch=master)](https://travis-ci.com/nicholasnjihian/amazin_aws_cli)


This repository is a command line application written in Rust that allows a company to take stock of its provisioned aws inventory from the terminal. Currently supported aws services :
1. S3 - Simple Storage Service
2. EC2 - Elastic Compute Cloud
3. ECS - 
4. ECR
5. RDS - Relational Database Service

The app uses the AWS SDK rusoto, which is based on a much lower level python AWS SDK called botocore (from which the popular python AWS SDK boto3 is based). This is possible because Rust can interface with python with various libraries like pyo3.

To start working with this app:
(i): Git clone the repo: 
$git clone https://github.com/nicholasnjihian/amazin_aws_cli.git

(ii): Then cd into the folder.

(iii): Then run:
$cargo build --release

(iv): Then run: 
$cargo run 

Note: The project is built in Rust so you have to have Rust installed on the computer. This can be done through the command: 
$curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
