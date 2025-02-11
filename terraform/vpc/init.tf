terraform {
  backend "s3" {
    bucket = "terraform-state-bobo12345"
    key    = "vpc/terraform.tfstate"
    region = "us-east-1"

    assume_role {
      role_arn = "arn:aws:iam::887998956998:role/GitHubActionsRole"
    }
  }
}

provider "aws" {
  region = var.region
}
