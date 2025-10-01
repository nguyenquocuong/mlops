terraform {
  backend "s3" {
    bucket  = "mla-c01-tf"
    key     = "1a-customer-churn-prediction-with-xgboost/terraform.tfstate"
    region  = "ap-southeast-1"
    encrypt = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }
  }
}
