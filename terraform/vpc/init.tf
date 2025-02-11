terraform {
  backend "s3" {
    bucket         = "terraform-state-bobo12345"
    key            = "vpc/terraform.tfstate"
    region         = "us-east-1"
    dynamodb_table = "fred-terraform-state01"
  }
}

provider "aws" {
  region = var.region
}
