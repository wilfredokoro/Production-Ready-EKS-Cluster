terraform {
  backend "s3" {
    bucket = "terraform-state-bobo1234567"
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
    bucket = "terraform-state-bobo1234567"
    key    = "vpc/terraform.tfstate"
    region = "us-east-1"
  }
}
