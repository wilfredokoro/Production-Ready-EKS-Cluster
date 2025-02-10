terraform {
  backend "s3" {
    bucket = "terraform-state-bobo12345"
    key    = "eks/terraform.tfstate"
    region = "us-east-1"
  }
}

provider "aws" {
  region = var.region
}

data "terraform_remote_state" "vpc" {
  backend = "s3"
  config = {
    bucket = "terraform-state-bobo12345"
    key    = "vpc/terraform.tfstate"
    region = "us-east-1"
  }
}
