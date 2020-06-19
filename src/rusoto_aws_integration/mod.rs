//!
//!This is the module that is responsible for all the queries and credentials
//!needed to get info about our cloud resources(s3,ec2,..so on).
//!This is the backbone of this application and it leverages the
//!rusoto crate which is an AWS SDK on top of another (python)AWS SDK called botocore.
//Imports for rusoto crate

use rusoto_core::request::{HttpClient, HttpConfig};
use rusoto_core::Region;
//credentials imports
use rusoto_credential::ProfileProvider;
//EC2
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client};
//ECS
use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest};
//S3
use rusoto_s3::{S3Client, S3};
//ECR
use rusoto_ecr::{DescribeRepositoriesRequest, Ecr, EcrClient};
//IAM
use rusoto_iam::{ListUsersRequest, Iam, IamClient};
//RDS
use rusoto_rds::{Rds,RdsClient,DescribeDBClustersMessage};

//Imports from the standard library
use std::error::Error;
use std::path::PathBuf;

//Bring get_region.rs into this namespace
pub mod get_region;

///This function will list all the s3 buckets provisioned
pub async fn list_s3_bucket<P>(aws_credentials_path: P, reg: Region) -> Result<(), Box<dyn Error>>
where
    P: Into<PathBuf>,
{
    let profile = ProfileProvider::with_default_configuration(aws_credentials_path);

    let s3client = S3Client::new_with(
        HttpClient::new().expect("Failed to create request dispatcher"),
        profile,
        reg,
    );
    match s3client.list_buckets().await {
        Err(err) => eprintln!("Error listing s3 buckets :{}", err),
        Ok(buckets) => println!("Buckets found : {:?}", buckets),
    };
    Ok(())
}

///Displays the ECS container clusters you've provisioned
pub async fn list_ecs_container_clusters<P>(credentials_file_path: P, reg: Region)
where
    P: Into<PathBuf>,
{
    // EcsClient configuration demonstrates setting the hyper read_buf_size option
    // to 2MB:
    let cred_provider = ProfileProvider::with_default_configuration(credentials_file_path);

    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let ecs = EcsClient::new_with(http_provider, cred_provider, reg);

    match ecs.list_clusters(ListClustersRequest::default()).await {
        Ok(clusters) => {
            for arn in clusters.cluster_arns.unwrap_or(vec![]) {
                println!("ecs arn -> {:?}", arn);
            }
        }
        Err(error) => {
            panic!("Error listing container instances {:#?}", error);
        }
    }
}

///Describe the EC2 instances provisioned with this function below
pub async fn describe_ec2<P>(credentials_file_path: P, reg: Region)
where
    P: Into<PathBuf>,
{
    let cred_provider = ProfileProvider::with_default_configuration(credentials_file_path);
    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let ec2 = Ec2Client::new_with(http_provider, cred_provider, reg);
    let req = DescribeInstancesRequest::default();
    match ec2.describe_instances(req).await {
        Ok(instances) => {
            for ec2_instance in instances.reservations.unwrap_or(vec![]) {
                println!("Instance in EC2 -> {:?}", ec2_instance);
            }
        }
        Err(error) => {
            panic!("Error listing ec2 instances {:#?}", error);
        }
    }
}

///Describe the RDS provisioned in your cloud
pub async fn describe_ecr<P>(cred_file_path: P, reg: Region) 
where P: Into<PathBuf>
{
    let cred_provider = ProfileProvider::with_default_configuration(cred_file_path);
    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let ecr_client = EcrClient::new_with(http_provider, cred_provider, reg);
    let req = DescribeRepositoriesRequest::default();
    match ecr_client.describe_repositories(req).await {
        Ok(response) => {
            for repo in response.repositories.unwrap_or(vec![]){
                println!("ECR repository == {:?}", repo);
            }
        },
        Err(error) => panic!("Error Listing ECR repos : {:#?}", error),

    }

}

///The function "describe_iam" describes the
///Identity and Access Management profiles, users and groups set up.
pub async fn describe_iam<P>(cred_file_path: P, reg: Region) 
where P : Into<PathBuf>
{
    let cred_provider = ProfileProvider::with_default_configuration(cred_file_path);
    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let iam = IamClient::new_with(http_provider, cred_provider, reg);
    let request = ListUsersRequest {
        ..Default::default()
    };
    match iam.list_users(request).await {
        Ok(response) => {
            for user in response.users{
                println!("IAM User == {:?}", user);
            }
        },
        Err(error) => panic!("Error Listing IAM users info : {:#?}", error),

    }

}

///The function "describe_ecr" will describe the ECR set up.
pub async fn describe_rds<P>(cred_file_path: P, reg: Region) 
where P: Into<PathBuf>
{
    let cred_provider = ProfileProvider::with_default_configuration(cred_file_path);
    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let rds_client = RdsClient::new_with(http_provider, cred_provider, reg);

    let request = DescribeDBClustersMessage::default();
    
    match rds_client.describe_db_clusters(request).await {
        Ok(response) => {
            for db in response.db_clusters.unwrap_or(vec![]){
                println!("RDS db == {:?}", db);
            }
        },
        Err(error) => panic!("Error Listing ECR repos : {:#?}", error),

    }

}
