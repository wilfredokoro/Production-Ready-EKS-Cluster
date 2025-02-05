terraform {
  backend "s3" {
    bucket = "terraform-state-2322122332"
    key    = "vpc/terraform.tfstate"
    region = "us-east-1"
  }
}

provider "aws" {
  region = var.region
}
