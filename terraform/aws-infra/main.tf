provider "aws" {
  region = "us-west-2"  # Change as per your region
}

module "vpc" {
  source = "./modules/vpc"
  vpc_cidr_block  = "10.0.0.0/16"  # or any appropriate CIDR block
}

module "eks" {
  source      = "./modules/eks"
  subnet_ids  = module.vpc.subnet_ids  # Pass subnet_ids here
  cluster_name = "my-cluster"  # Pass the cluster name here
}

module "iam" {
  source = "./modules/iam"
}

module "autoscaling" {
  source = "./modules/autoscaling"
  subnet_ids  = module.vpc.subnet_ids
}
