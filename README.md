[![Build Status](https://travis-ci.com/nicholasnjihian/amazin_aws_cli.svg?branch=master)](https://travis-ci.com/nicholasnjihian/amazin_aws_cli)


AMAZIN AWS CLI IN RUST
----------------------
----------------------

This repository is a command line application written in Rust that allows a company to take stock of its provisioned aws inventory from the terminal. Currently supported aws services :
1. *S3* - Simple Storage Service
2. *EC2* - Elastic Compute Cloud
3. *ECS* - Elastic Container Service
4. *ECR* - Elastic Container Registry
5. RDS - Relational Database Service

The app uses the AWS SDK rusoto, which is based on a much lower level python AWS SDK called botocore (from which the popular python AWS SDK boto3 is based). This is possible because Rust can interface with python with various libraries like pyo3.
In 2020, AWS has been hiring Rust engineers to work on a pure rust SDK however, as has been observed by the rusoto developers:https://matthewkmayer.github.io/blag/public/post/farewell-rusoto/.


***To start working with this app:***
## INSTRUCTIONS FOR SETTING UP ON LOCAL MACHINE.

**(i): Git clone the repo:**
$git clone https://github.com/nicholasnjihian/amazin_aws_cli.git

**(ii): Then cd into the cloned folder on your machine.**

Find a startup.sh which will help in quickly installing dependencies, building and running the application.This bash script will check whether you have cargo and rustup installed and install them for you if you do not. It will then display the versions of these 2 software packages and then build and run the crate/project/application.

**You can execute it by running:**
**./startup.sh**
---
It will show the necessary help, arguments and subcommands available.
Hence you can run with commands like:
*$./startup.sh --help*
*$./startup.sh configure*
*$./startup.sh configure -f <enter credentials file>*
*$./startup.sh list s3*
*$./startup.sh ls s3*
*$/.startup.sh ls ec2*
---

## Optional.
Optionally if you don't want to use the script above (if you're wary of internet-sourced scripts) you can just build the crate manually as follows(which is also what the script does).

**(i): To build the crate:(ensure you are in the same directory as the src directory):**
*$cargo build --release*

**(ii): To run:(ensure you are in the same directory as the src directory): **
*$cargo run <options/subcommand/arguments>*

*$ cargo run configure -f <enter credentials file>*
*$ cargo configure*
*$ cargo ls s3*
*$ cargo list s3*
*$ cargo ls ec2*
*$ cargo list ecs*

### (iii). You can generate the docs for this application by running(ensure you are in the same directory as the src directory):
*$ cargo docs --open*
---
***Note:*** **The project is built in Rust so you have to have Rust installed on the computer. This can be done through the command: **
*$curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh*
However, the bash script above installs this for you if you don't have it installed.

**Also the rusoto crate(crate is the name for Rust packages) requires that OpenSSL be installed for Linux. This can be done via :**
*$ sudo apt install openssl*
*$ sudo pacman -S openssl*
**and so on depending on your distribution.**



